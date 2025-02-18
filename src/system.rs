use anyhow::{anyhow, Result};
use std::{process::Command, path::PathBuf};
use which::which;
use dirs::home_dir;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fs;

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
        self.cache_dir.join(format!("{}_{}}", os, package))
    }

    pub fn is_fresh(&self, os: &str, package: &str) -> bool {
        let cache_path = self.get_cache_path(os, package);
        if let Ok(metadata) = fs::metadata(cache_path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(duration);
                    return now.as_secs() - duration.as_secs() < CACHE_DURATION_SECS;
                }
            }
        }
        false
    }

    pub fn update(&self, os: &str, package: &str) -> Result<()> {
        let cache_path = self.get_cache_path(os, package);
        fs::write(cache_path, SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
            .to_string())?;
        Ok(())
    }
}

pub fn retry_with_backoff<F, T>(mut f: F, retries: u32) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    let mut attempts = 0;
    let mut last_error = None;

    while attempts < retries {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                attempts += 1;
                if attempts < retries {
                    let delay = std::time::Duration::from_secs(2u64.pow(attempts));
                    std::thread::sleep(delay);
                }
            }
        }
    }
    Err(last_error.unwrap_or_else(|| anyhow!("Retry failed")))
}

pub fn ensure_sudo_access() -> Result<()> {
    if whoami::username() == "root" {
        return Ok(());
    }

    if which("sudo").is_err() {
        return Err(anyhow!("sudo is not installed. Please run as root or install sudo."));
    }

    match run_cmd("sudo", &["-n", "true"]) {
        Ok(_) => println!("Sudo access verified"),
        Err(_) => {
            println!("Please enter your sudo password (will be cached)");
            run_cmd("sudo", &["-v"])?;
            run_cmd("bash", &["-c", "while true; do sudo -n true; sleep 50; kill -0 $$ || exit; done 2>/dev/null &"])?;
        }
    }
    Ok(())
}

pub fn detect_platform() -> Result<(String, String)> {
    let arch = std::env::consts::ARCH.to_string();
    
    match std::env::consts::OS {
        "macos" => Ok(("macos".to_string(), arch)),
        "linux" => {
            let distro = if std::fs::read_to_string("/etc/os-release")?
                .lines()
                .any(|line| line.contains("ID=") && (line.contains("amzn") || line.contains("amazon")))
            {
                "amazon"
            } else {
                "ubuntu"
            };
            Ok((distro.to_string(), arch))
        }
        os => Err(anyhow!("Unsupported OS: {}", os))
    }
}

pub fn setup(os: &str) -> Result<()> {
    match os {
        "ubuntu" => {
            run_cmd("sudo", &["apt-get", "update"])?;
            run_cmd("sudo", &["apt-get", "upgrade", "-y"])?;
        }
        "amazon" => {
            run_cmd("sudo", &["yum", "update", "-y"])?;
        }
        "macos" => {
            if which("brew").is_err() {
                run_cmd("bash", &["-c", "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""])?;
            }
            run_cmd("brew", &["update"])?;
        }
        _ => return Err(anyhow!("Unsupported OS: {}", os))
    }
    Ok(())
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|_| ())
        .map_err(|e| anyhow!("Failed to run {}: {}", cmd, e))
}
