use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub jwt: String,
    pub identity: Identity,
    pub saved_at: i64,
    pub base_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identity {
    pub user_id: String,
    pub identity: String,
    pub auth_method: String,
    pub tier: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtPayloadDisplay {
    #[serde(default)]
    pub exp: i64,
    #[serde(default)]
    pub scopes: Vec<String>,
}

fn rokha_home() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".rokha")
}

pub fn credentials_path() -> PathBuf {
    rokha_home().join("credentials")
}

pub fn load() -> Option<Credentials> {
    let path = credentials_path();
    let body = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&body).ok()
}

pub fn save(creds: &Credentials) -> std::io::Result<()> {
    let dir = rokha_home();
    std::fs::create_dir_all(&dir)?;
    let path = credentials_path();
    let body = serde_json::to_string_pretty(creds).map_err(std::io::Error::other)?;
    std::fs::write(&path, body)?;
    set_owner_only(&path)?;
    Ok(())
}

pub fn delete() -> std::io::Result<bool> {
    let path = credentials_path();
    if path.exists() {
        std::fs::remove_file(&path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(unix)]
fn set_owner_only(path: &std::path::Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o600);
    std::fs::set_permissions(path, perms)
}

#[cfg(not(unix))]
fn set_owner_only(_path: &std::path::Path) -> std::io::Result<()> {
    Ok(())
}

/// Decode the JWT payload without verifying the signature. Display-only —
/// `ro whoami` shows what claims the token *says* it has; the server
/// is the only authority on whether they're valid.
pub fn decode_payload(jwt: &str) -> Option<JwtPayloadDisplay> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    let bytes = URL_SAFE_NO_PAD.decode(parts[1]).ok()?;
    serde_json::from_slice(&bytes).ok()
}
