use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, fs};
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct PackageCache {
    last_checked: DateTime<Utc>,
    version: String,
}

/// Trait for implementing a package manager
/// 
/// # Implementation Requirements
/// 1. All operations must be idempotent
/// 2. Version checks must use caching (24h)
/// 3. Directory creation must use safe permissions
/// 4. System package managers must handle sudo
/// 5. All errors must be properly propagated
pub trait PackageManager {
    /// Get the name of this package manager (e.g., "cargo", "apt", "brew")
    /// Get the name of this package manager
    /// Used for cache directory naming and logging
    fn name(&self) -> &str;

    /// Check if a package is installed
    /// Check if a package is installed
    /// Must be fast and not require network access
    fn is_installed(&self, package: &str) -> Result<bool>;

    /// Get the latest available version of a package
    /// Get the latest available version of a package
    /// Must implement caching via check_cache/update_cache
    fn get_latest_version(&self, package: &str) -> Result<String>;

    /// Install a package
    /// Install a package
    /// Must handle privilege escalation if needed
    fn install(&self, package: &str) -> Result<()>;

    /// Force upgrade a package
    /// Force upgrade a package
    /// Must handle privilege escalation if needed
    fn upgrade(&self, package: &str) -> Result<()>;

    /// Get cache directory for this package manager
    fn cache_dir(&self) -> PathBuf {
        let base = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        let mut path = base.join("cyrup").join(self.name());
        
        // Create with safe permissions if doesn't exist
        if !path.exists() {
            fs::create_dir_all(&path)
                .and_then(|_| fs::set_permissions(&path, fs::Permissions::from_mode(0o755)))
                .unwrap_or_else(|_| {
                    path = PathBuf::from("/tmp").join("cyrup").join(self.name());
                    fs::create_dir_all(&path).unwrap();
                });
        }
        
        path
    }

    /// Check cache for package version
    fn check_cache(&self, package: &str) -> Option<PackageCache> {
        let cache_file = self.cache_dir().join(format!("{}.json", package));
        if let Ok(contents) = fs::read_to_string(cache_file) {
            if let Ok(cache) = serde_json::from_str::<PackageCache>(&contents) {
                if Utc::now() - cache.last_checked < Duration::days(1) {
                    return Some(cache);
                }
            }
        }
        None
    }

    /// Update cache for package
    fn update_cache(&self, package: &str, version: String) -> Result<()> {
        let cache = PackageCache {
            last_checked: Utc::now(),
            version,
        };
        let cache_dir = self.cache_dir();
        fs::create_dir_all(&cache_dir)?;
        fs::write(
            cache_dir.join(format!("{}.json", package)),
            serde_json::to_string(&cache)?,
        )?;
        Ok(())
    }

    /// Install or upgrade a package following our standard logic
    fn install_or_upgrade(&self, package: &str, force_upgrade: bool) -> Result<()> {
        // 1. Check if installed
        if !self.is_installed(package)? {
            println!("Installing {package} using {}", self.name());
            return self.install(package);
        }

        // 2. Check cache if not forcing upgrade
        if !force_upgrade {
            if let Some(cache) = self.check_cache(package) {
                println!("Using cached version check for {package}");
                return Ok(());
            }
        }

        // 3. Check for newer version
        let latest = self.get_latest_version(package)?;
        self.update_cache(package, latest.clone())?;

        // 4. Upgrade if needed
        if force_upgrade {
            println!("Force upgrading {package} to {latest}");
            self.upgrade(package)?;
        }

        Ok(())
    }
}

pub struct CargoManager;

#[async_trait::async_trait]
impl PackageManager for CargoManager {
    fn name(&self) -> &str {
        "cargo"
    }

    fn is_installed(&self, package: &str) -> Result<bool> {
        Ok(Command::new("cargo")
            .arg("install")
            .arg("--list")
            .output()
            .await?
            .stdout
            .lines()
            .any(|line| line.contains(package)))
    }

    fn get_latest_version(&self, package: &str) -> Result<String> {
        // First check cache
        if let Some(cache) = self.check_cache(package) {
            return Ok(cache.version);
        }

        // Get version from cargo
        let output = Command::new("cargo")
            .arg("search")
            .arg(package)
            .arg("--limit")
            .arg("1")
            .output()
            .await?;
        
        let version = String::from_utf8(output.stdout)?
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(2))
            .ok_or_else(|| anyhow::anyhow!("Could not parse version for {}", package))?
            .trim_matches('"')
            .to_string();
        
        // Update cache
        self.update_cache(package, version.clone())?;
        
        Ok(version)
    }

    fn install(&self, package: &str) -> Result<()> {
        Command::new("cargo")
            .arg("install")
            .arg(package)
            .status()
            .await?;
        Ok(())
    }

    fn upgrade(&self, package: &str) -> Result<()> {
        Command::new("cargo")
            .arg("install")
            .arg("-f")
            .arg(package)
            .status()
            .await?;
        Ok(())
    }
}

pub struct AptManager;

#[async_trait::async_trait]
impl PackageManager for AptManager {
    fn name(&self) -> &str {
        "apt"
    }

    fn is_installed(&self, package: &str) -> Result<bool> {
        Ok(Command::new("dpkg")
            .arg("-l")
            .arg(package)
            .status()
            .await?
            .success())
    }

    fn get_latest_version(&self, package: &str) -> Result<String> {
        // First check cache
        if let Some(cache) = self.check_cache(package) {
            return Ok(cache.version);
        }

        // Get version from apt
        let output = Command::new("apt-cache")
            .arg("policy")
            .arg(package)
            .output()
            .await?;

        let version = String::from_utf8_lossy(&output.stdout)
            .lines()
            .find(|line| line.contains("Candidate:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .ok_or_else(|| anyhow::anyhow!("Could not parse version for {}", package))?
            .to_string();

        // Update cache
        self.update_cache(package, version.clone())?;

        Ok(version)
    }

    fn install(&self, package: &str) -> Result<()> {
        Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .arg(package)
            .status()
            .await?;
        Ok(())
    }

    fn upgrade(&self, package: &str) -> Result<()> {
        Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("--only-upgrade")
            .arg("-y")
            .arg(package)
            .status()
            .await?;
        Ok(())
    }
}

pub struct BrewManager;

#[async_trait::async_trait]
impl PackageManager for BrewManager {
    fn name(&self) -> &str {
        "brew"
    }

    fn is_installed(&self, package: &str) -> Result<bool> {
        Ok(Command::new("brew")
            .arg("list")
            .arg(package)
            .status()
            .await?
            .success())
    }

    fn get_latest_version(&self, package: &str) -> Result<String> {
        // First check cache
        if let Some(cache) = self.check_cache(package) {
            return Ok(cache.version);
        }

        // Get version from brew
        let output = Command::new("brew")
            .arg("info")
            .arg(package)
            .output()
            .await?;
        
        let version = String::from_utf8(output.stdout)?
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .ok_or_else(|| anyhow::anyhow!("Could not parse version for {}", package))?
            .to_string();
        
        // Update cache
        self.update_cache(package, version.clone())?;
        
        Ok(version)
    }

    fn install(&self, package: &str) -> Result<()> {
        Command::new("brew")
            .arg("install")
            .arg(package)
            .status()
            .await?;
        Ok(())
    }

    fn upgrade(&self, package: &str) -> Result<()> {
        Command::new("brew")
            .arg("upgrade")
            .arg(package)
            .status()
            .await?;
        Ok(())
    }
}

/// Get the appropriate package manager for the current platform
pub fn get_package_manager(package_type: &str) -> Box<dyn PackageManager> {
    match package_type {
        "cargo" => Box::new(CargoManager),
        "apt" => Box::new(AptManager),
        "brew" => Box::new(BrewManager),
        _ => panic!("Unsupported package manager: {}", package_type),
    }
}
