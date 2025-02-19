use anyhow::{anyhow, Result};
use dirs::home_dir;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{path::PathBuf, process::Command};
use which::which;

pub const RETRY_ATTEMPTS: u32 = 3;
pub const CACHE_DURATION_SECS: u64 = 24 * 60 * 60; // 24 hours

pub struct PackageCache {
    cache_dir: PathBuf,
}

impl PackageCache {
    pub fn new() -> Result<Self> {
        let cache_dir = home_dir()
            .ok_or_else(|| anyhow!("Cannot determine home directory"))?
            .join(".cache/cypackages");
        fs::create_dir_all(&cache_dir)?;
        Ok(Self { cache_dir })
    }

    pub fn get_cache_path(&self, os: &str, package: &str) -> PathBuf {
        self.cache_dir.join(format!("{}_{}", os, package))
    }

    pub fn is_fresh(&self, os: &str, package: &str) -> bool {
        let cache_path = self.get_cache_path(os, package);
        if !cache_path.exists() {
            return false;
        }

        let metadata = match fs::metadata(&cache_path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let modified = metadata
            .modified()
            .unwrap_or(UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        now.as_secs() - modified.as_secs() < CACHE_DURATION_SECS
    }

    pub fn update(&self, os: &str, package: &str) -> Result<()> {
        let cache_path = self.get_cache_path(os, package);
        fs::write(&cache_path, "")?;
        Ok(())
    }
}

/// Retry a function with exponential backoff
pub async fn retry_with_backoff<F, T, E>(mut f: F, retries: u32) -> Result<T>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut attempts = 0;
    loop {
        match f() {
            Ok(value) => return Ok(value),
            Err(e) => {
                attempts += 1;
                if attempts >= retries {
                    return Err(anyhow!("Failed after {} attempts: {}", attempts, e));
                }
                let delay = Duration::from_millis(2u64.pow(attempts) * 100);
                tokio::time::sleep(delay).await;
            }
        }
    }
}

/// Ensure sudo access is available and cached
pub fn ensure_sudo_access() -> Result<()> {
    // Check if sudo is available
    which("sudo").map_err(|_| anyhow!("sudo is not available"))?;

    // Try to get cached sudo credentials
    let output = Command::new("sudo")
        .arg("-n")
        .arg("true")
        .output()
        .map_err(|e| anyhow!("Failed to execute sudo: {}", e))?;

    if !output.status.success() {
        // Need to prompt for password
        println!("⚡ Sudo access required for system setup");
        println!("🔐 Please enter your password when prompted");

        let status = Command::new("sudo")
            .arg("true")
            .status()
            .map_err(|e| anyhow!("Failed to execute sudo: {}", e))?;

        if !status.success() {
            return Err(anyhow!("Failed to obtain sudo access"));
        }
    }

    Ok(())
}

/// Detect the current platform (OS and architecture)
pub fn detect_platform() -> Result<(String, String)> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    // Map OS names to our standardized names
    let os_name = match os {
        "linux" => "linux",
        "macos" => "darwin",
        "windows" => "windows",
        _ => return Err(anyhow!("Unsupported OS: {}", os)),
    };

    // Map architecture names to our standardized names
    let arch_name = match arch {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => return Err(anyhow!("Unsupported architecture: {}", arch)),
    };

    Ok((os_name.to_string(), arch_name.to_string()))
}
