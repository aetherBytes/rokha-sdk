//! Local audio for `ro voice` (feature = "voice").
//!
//! The ONLY genuinely-local pieces of the voice loop: capture the microphone
//! and play audio back. STT, TTS, and the agent all run on the platform — this
//! file holds no intelligence, just PortAudio-style plumbing over `cpal`
//! (capture) and `rodio` (playback).
//!
//! Interaction model v1: push-to-talk with trailing-silence auto-stop. The
//! caller starts a recording (after the user presses Enter); we stop it after
//! ~1s of silence once speech has been heard. Full-duplex VAD + barge-in is the
//! documented Phase-2 upgrade (mirrors the browser's own phased rollout).
//!
//! cpal/rodio streams are `!Send`, so both entry points are BLOCKING and are
//! built + dropped entirely within one thread — call them via
//! `tokio::task::spawn_blocking`.

use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// A local-audio failure. `String`-backed so it is `Send` (crosses the
/// spawn_blocking boundary) and prints a plain, honest message.
#[derive(Debug)]
pub struct AudioError(pub String);

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Record one utterance from the default input device, returning 16-bit mono
/// WAV bytes ready to POST to `/api/voice/stt`. Stops after ~1s of trailing
/// silence (once speech has been detected) or a hard 30s cap; errors if no
/// speech arrives within 8s. BLOCKING.
pub fn record_utterance() -> Result<Vec<u8>, AudioError> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| AudioError("no microphone (default input device) found".into()))?;
    let supported = device
        .default_input_config()
        .map_err(|e| AudioError(format!("could not read microphone config: {e}")))?;
    let sample_format = supported.sample_format();
    let sample_rate = supported.sample_rate().0;
    let channels = supported.channels() as usize;
    let config: cpal::StreamConfig = supported.into();

    // Mono f32 samples, appended by the audio callback.
    let samples: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let err_fn = |e| eprintln!("\x1b[2m(audio stream error: {e})\x1b[0m");

    let stream = {
        let buf = samples.clone();
        let build = match sample_format {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| append_mono(&buf, data, channels),
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let f: Vec<f32> = data.iter().map(|s| *s as f32 / 32768.0).collect();
                    append_mono(&buf, &f, channels);
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    let f: Vec<f32> = data.iter().map(|s| (*s as f32 / 32768.0) - 1.0).collect();
                    append_mono(&buf, &f, channels);
                },
                err_fn,
                None,
            ),
            other => return Err(AudioError(format!("unsupported sample format {other:?}"))),
        };
        build.map_err(|e| AudioError(format!("could not open microphone stream: {e}")))?
    };
    stream
        .play()
        .map_err(|e| AudioError(format!("could not start microphone: {e}")))?;

    // Trailing-silence auto-stop. Calibrate a noise floor from the first 300ms,
    // then stop after `hang` of sub-threshold audio once speech has been heard.
    let frame = (sample_rate as usize / 50).max(160); // ~20ms
    let hang = Duration::from_millis(1000);
    let max = Duration::from_secs(30);
    let calibrate = Duration::from_millis(300);

    let start = Instant::now();
    let mut cursor = 0usize;
    let mut floor = 0.0f32;
    let mut floor_n = 0u32;
    let mut heard_speech = false;
    let mut last_voice = Instant::now();

    loop {
        std::thread::sleep(Duration::from_millis(20));
        let len = { samples.lock().unwrap().len() };
        while cursor + frame <= len {
            let r = {
                let s = samples.lock().unwrap();
                rms(&s[cursor..cursor + frame])
            };
            cursor += frame;
            if start.elapsed() < calibrate {
                floor = (floor * floor_n as f32 + r) / (floor_n + 1) as f32;
                floor_n += 1;
                continue;
            }
            let threshold = (floor * 3.5).max(0.012);
            if r > threshold {
                heard_speech = true;
                last_voice = Instant::now();
            }
        }
        if heard_speech && last_voice.elapsed() >= hang {
            break;
        }
        if start.elapsed() >= max {
            break;
        }
        if !heard_speech && start.elapsed() >= Duration::from_secs(8) {
            drop(stream);
            return Err(AudioError("no speech detected".into()));
        }
    }
    drop(stream); // stop capture

    let pcm = { samples.lock().unwrap().clone() };
    encode_wav(&pcm, sample_rate)
}

/// Decode + play mp3 bytes (what `/api/voice/tts` returns) to the default
/// output device, blocking until playback finishes. BLOCKING.
pub fn play_mp3(bytes: Vec<u8>) -> Result<(), AudioError> {
    let (_stream, handle) = rodio::OutputStream::try_default()
        .map_err(|e| AudioError(format!("no audio output device: {e}")))?;
    let sink =
        rodio::Sink::try_new(&handle).map_err(|e| AudioError(format!("audio sink: {e}")))?;
    let decoder = rodio::Decoder::new(Cursor::new(bytes))
        .map_err(|e| AudioError(format!("could not decode audio: {e}")))?;
    sink.append(decoder);
    sink.sleep_until_end();
    Ok(())
}

fn append_mono(buf: &Arc<Mutex<Vec<f32>>>, data: &[f32], channels: usize) {
    let mut b = buf.lock().unwrap();
    if channels <= 1 {
        b.extend_from_slice(data);
    } else {
        for frame in data.chunks(channels) {
            let sum: f32 = frame.iter().sum();
            b.push(sum / channels as f32);
        }
    }
}

fn rms(s: &[f32]) -> f32 {
    if s.is_empty() {
        return 0.0;
    }
    let sum: f32 = s.iter().map(|x| x * x).sum();
    (sum / s.len() as f32).sqrt()
}

fn encode_wav(pcm: &[f32], sample_rate: u32) -> Result<Vec<u8>, AudioError> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut cursor = Cursor::new(Vec::<u8>::new());
    {
        let mut writer = hound::WavWriter::new(&mut cursor, spec)
            .map_err(|e| AudioError(format!("wav init: {e}")))?;
        for &s in pcm {
            let v = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            writer
                .write_sample(v)
                .map_err(|e| AudioError(format!("wav write: {e}")))?;
        }
        writer
            .finalize()
            .map_err(|e| AudioError(format!("wav finalize: {e}")))?;
    }
    Ok(cursor.into_inner())
}
