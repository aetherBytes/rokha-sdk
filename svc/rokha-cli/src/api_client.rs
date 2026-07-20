use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: &str = "1.0.0";

/// User-Agent sent on every request. The rokha.ai edge (WAF) rejects requests
/// with no User-Agent, so an explicit one is mandatory — without it the whole
/// CLI 403s against prod. Keep it identifiable.
pub const USER_AGENT: &str = concat!("ro/", env!("CARGO_PKG_VERSION"), " (+https://rokha.ai)");

/// Build the shared HTTP client with the mandatory User-Agent. Use this
/// everywhere instead of `reqwest::Client::new()`.
pub fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

/// One Rokha Registry listing. Lenient by design — the registry spans nine
/// providers with uneven fields, so everything is optional and nulls are
/// tolerated. Mirrors an item from `GET /api/marketplace/registry`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Listing {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub provider_id: Option<String>,
    pub external_id: Option<String>,
    pub listing_type: Option<String>,
    pub execution_class: Option<String>,
    pub tags: Vec<String>,
    pub downloads: Option<i64>,
    pub stars: Option<i64>,
    pub rokha_runs: Option<i64>,
    pub probe_verified: Option<bool>,
}

impl Listing {
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("(unnamed)")
    }
    pub fn class(&self) -> &str {
        self.execution_class
            .as_deref()
            .or(self.listing_type.as_deref())
            .unwrap_or("—")
    }
    pub fn provider(&self) -> &str {
        self.provider_id.as_deref().unwrap_or("—")
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct RegistryPage {
    pub items: Vec<Listing>,
    pub total_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct SchemaVersion {
    pub version: String,
}

pub struct RokhaClient {
    http: reqwest::Client,
    base_url: String,
}

impl RokhaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: http_client(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn schema_version(&self) -> Result<SchemaVersion, reqwest::Error> {
        self.http
            .get(format!("{}/api/schema/version", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    /// Search the live Rokha Registry. `search` empty = browse newest. Hits
    /// `GET /api/marketplace/registry` — the real, populated endpoint (the old
    /// `/api/marketplace/listings` is a stub that always returns empty).
    pub async fn search_registry(
        &self,
        search: &str,
        limit: i64,
    ) -> Result<RegistryPage, reqwest::Error> {
        // The endpoint browses all listings when `search` is OMITTED; an empty
        // `search=` returns zero. So only send the param when we have a query.
        let mut q: Vec<(&str, String)> = vec![("limit", limit.to_string())];
        if !search.is_empty() {
            q.push(("search", search.to_string()));
        }
        self.http
            .get(format!("{}/api/marketplace/registry", self.base_url))
            .query(&q)
            .send()
            .await?
            .json()
            .await
    }

    /// Convenience: registry listings for a query (or newest when empty).
    pub async fn list_tools(&self) -> Result<Vec<Listing>, reqwest::Error> {
        Ok(self.search_registry("", 50).await?.items)
    }

    /// Live agent status (health, active model, tool count). Returned as a raw
    /// JSON value so the caller renders whatever fields the server exposes.
    pub async fn agent_status(&self) -> Result<serde_json::Value, reqwest::Error> {
        self.http
            .get(format!("{}/api/agents/rokha-agent/status", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    /// Forward one JSON-RPC envelope to the platform MCP endpoint
    /// (`/mcp/jsonrpc`). The local `ro mcp serve` bridge is a thin passthrough:
    /// the platform holds the tools, the registry, and the agent — we just
    /// carry the bytes. A bearer token (when logged in) unlocks the authed
    /// tools; without it the public tool set answers.
    pub async fn mcp_forward(
        &self,
        body: &serde_json::Value,
        jwt: Option<&str>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let mut req = self
            .http
            .post(format!("{}/mcp/jsonrpc", self.base_url))
            .json(body);
        if let Some(token) = jwt {
            req = req.bearer_auth(token);
        }
        req.send().await?.json().await
    }

    // --- Voice doors (GIVE_RO_A_VOICE). Thin transport only: the platform runs
    //     Deepgram STT + ElevenLabs TTS server-side; `ro` just moves the bytes. --

    /// `GET /api/voice/health` (public) — reports whether STT/TTS are configured.
    #[cfg(feature = "voice")]
    pub async fn voice_health(&self) -> Result<serde_json::Value, reqwest::Error> {
        self.http
            .get(format!("{}/api/voice/health", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    /// `POST /api/voice/stt` — WAV bytes in, transcript out. JWT + paid tier.
    #[cfg(feature = "voice")]
    pub async fn voice_stt(&self, wav: Vec<u8>, jwt: &str) -> Result<String, VoiceError> {
        let resp = self
            .http
            .post(format!("{}/api/voice/stt", self.base_url))
            .bearer_auth(jwt)
            .header(reqwest::header::CONTENT_TYPE, "audio/wav")
            .body(wav)
            .send()
            .await
            .map_err(VoiceError::transport)?;
        let status = resp.status().as_u16();
        let json: serde_json::Value = resp.json().await.map_err(VoiceError::transport)?;
        if !(200..300).contains(&status) {
            return Err(VoiceError::from_body(status, &json));
        }
        Ok(json
            .get("transcript")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    /// `POST /api/voice/tts` — text in, mp3 bytes out. JWT + paid tier.
    #[cfg(feature = "voice")]
    pub async fn voice_tts(&self, text: &str, jwt: &str) -> Result<Vec<u8>, VoiceError> {
        let resp = self
            .http
            .post(format!("{}/api/voice/tts", self.base_url))
            .bearer_auth(jwt)
            .json(&serde_json::json!({ "text": text }))
            .send()
            .await
            .map_err(VoiceError::transport)?;
        let status = resp.status().as_u16();
        if !(200..300).contains(&status) {
            let json: serde_json::Value = resp.json().await.unwrap_or(serde_json::Value::Null);
            return Err(VoiceError::from_body(status, &json));
        }
        Ok(resp.bytes().await.map_err(VoiceError::transport)?.to_vec())
    }
}

/// A voice-door failure carrying the HTTP status so the caller can tell a
/// tier/allowance limit (402/403/429 — trigger the loud upsell) from a plain
/// error or an honest 503 (keys unset).
#[cfg(feature = "voice")]
#[derive(Debug)]
pub struct VoiceError {
    pub status: u16,
    pub message: String,
}

#[cfg(feature = "voice")]
impl VoiceError {
    fn transport(e: reqwest::Error) -> Self {
        VoiceError {
            status: 0,
            message: format!("voice request failed: {e}"),
        }
    }
    fn from_body(status: u16, json: &serde_json::Value) -> Self {
        let message = json
            .get("message")
            .or_else(|| json.get("error"))
            .and_then(|v| v.as_str())
            .unwrap_or("voice error")
            .to_string();
        VoiceError { status, message }
    }
    /// A tier/allowance denial — the moment to upsell.
    pub fn is_limit(&self) -> bool {
        matches!(self.status, 402 | 403 | 429)
    }
    /// The server has no keys configured (Run-Real-or-Raise honest 503).
    pub fn is_unconfigured(&self) -> bool {
        self.status == 503
    }
}
