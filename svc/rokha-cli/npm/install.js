#!/usr/bin/env node
// Postinstall: downloads the matching `ro` binary from the GitHub
// release for this @rokha/cli version, verifies sha256, and places
// it at `vendor/ro` so the `bin/ro` shim can exec it.

'use strict';

const fs = require('fs');
const path = require('path');
const https = require('https');
const crypto = require('crypto');
const os = require('os');
const { spawnSync } = require('child_process');

const REPO = process.env.RO_REPO || 'aetherBytes/rokha-sdk';
const pkgRoot = path.resolve(__dirname, '..');
const pkg = require(path.join(pkgRoot, 'package.json'));
const version = pkg.version;
const tag = `cli-v${version}`;

function targetTriple() {
  const platform = process.platform;
  const arch = process.arch;
  if (platform === 'darwin' && arch === 'arm64') return 'aarch64-apple-darwin';
  if (platform === 'darwin' && arch === 'x64') return 'x86_64-apple-darwin';
  if (platform === 'linux' && arch === 'x64') return 'x86_64-unknown-linux-gnu';
  if (platform === 'linux' && arch === 'arm64') return 'aarch64-unknown-linux-gnu';
  return null;
}

function log(msg) { process.stderr.write(`@rokha/cli: ${msg}\n`); }

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const get = (u, redirectsLeft = 5) => {
      https.get(u, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          if (redirectsLeft <= 0) return reject(new Error('too many redirects'));
          return get(res.headers.location, redirectsLeft - 1);
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`download ${u} -> HTTP ${res.statusCode}`));
        }
        const file = fs.createWriteStream(dest);
        res.pipe(file);
        file.on('finish', () => file.close(() => resolve()));
        file.on('error', reject);
      }).on('error', reject);
    };
    get(url);
  });
}

function sha256(file) {
  const hash = crypto.createHash('sha256');
  hash.update(fs.readFileSync(file));
  return hash.digest('hex');
}

async function main() {
  // Honor CI / dev opt-outs.
  if (process.env.ROKHA_CLI_SKIP_INSTALL === '1') {
    log('skip (ROKHA_CLI_SKIP_INSTALL=1)');
    return;
  }

  const target = targetTriple();
  if (!target) {
    log(`unsupported platform: ${process.platform}/${process.arch}`);
    log('use cargo install rokha-cli, or see https://github.com/aetherBytes/rokha-sdk');
    process.exit(0); // soft-fail so npm install completes
  }

  const asset = `ro-${version}-${target}.tar.gz`;
  const baseUrl = `https://github.com/${REPO}/releases/download/${tag}/${asset}`;
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'rokha-cli-'));
  const tarPath = path.join(tmpDir, asset);

  log(`downloading ${asset}`);
  try {
    await download(baseUrl, tarPath);
  } catch (e) {
    log(`download failed: ${e.message}`);
    log(`(release ${tag} may not exist yet, or the network blocked the request)`);
    process.exit(0);
  }

  // Optional sha256 verify
  const shaPath = `${tarPath}.sha256`;
  try {
    await download(`${baseUrl}.sha256`, shaPath);
    const expected = fs.readFileSync(shaPath, 'utf8').trim().split(/\s+/)[0];
    const actual = sha256(tarPath);
    if (expected && expected !== actual) {
      log(`checksum mismatch: expected ${expected}, got ${actual}`);
      process.exit(1);
    }
    log('checksum verified');
  } catch (_) {
    log('no checksum sidecar — skipping verify');
  }

  // Untar with system tar (avoid bundling a tar lib).
  const vendorDir = path.join(pkgRoot, 'vendor');
  fs.mkdirSync(vendorDir, { recursive: true });
  const untar = spawnSync('tar', ['-xzf', tarPath, '-C', tmpDir], { stdio: 'inherit' });
  if (untar.status !== 0) {
    log('tar extract failed');
    process.exit(1);
  }

  const innerDir = path.join(tmpDir, `ro-${version}-${target}`);
  const binSrc = path.join(innerDir, 'ro');
  const binDest = path.join(vendorDir, 'ro');
  fs.copyFileSync(binSrc, binDest);
  fs.chmodSync(binDest, 0o755);

  log(`installed -> ${binDest}`);
}

main().catch((e) => {
  log(`unexpected error: ${e.message}`);
  process.exit(0); // soft-fail
});
