use crate::config::Config;

pub fn show(cfg: &Config) -> i32 {
    println!("base url:    {}", cfg.erebus_url);
    println!("config file: {}", Config::config_path().display());
    let env_override = std::env::var("ROKHA_BASE_URL")
        .ok()
        .filter(|v| !v.is_empty());
    if let Some(env) = env_override {
        println!();
        println!("(ROKHA_BASE_URL env override is active: {})", env);
    }
    0
}

pub fn set_base_url(url: &str) -> i32 {
    let trimmed = url.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        eprintln!("ro config set-base-url: url is empty");
        return 1;
    }
    if !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        eprintln!("ro config set-base-url: url must start with http:// or https://");
        return 1;
    }
    let cfg = Config {
        erebus_url: trimmed.to_string(),
    };
    match cfg.save() {
        Ok(path) => {
            println!("✓ base url set to {}", cfg.erebus_url);
            println!("  saved to {}", path.display());
            0
        }
        Err(e) => {
            eprintln!("ro config set-base-url: write failed: {}", e);
            1
        }
    }
}
