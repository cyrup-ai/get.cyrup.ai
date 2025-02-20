use anyhow::{anyhow, Result};
use std::fmt;
use which::which;

use super::recipe::Recipe;
use super::Package;

/// Supported package managers
#[derive(Debug, Clone, PartialEq)]
pub enum PackageManager {
    /// Debian-based systems (Debian, Ubuntu, etc.)
    Apt,
    /// RPM-based systems (RHEL, CentOS, Amazon Linux)
    Yum,
    /// macOS package manager
    Brew,
}

impl PackageManager {
    /// Detect the system's package manager
    pub fn detect() -> Result<Self> {
        if cfg!(target_os = "macos") {
            if which("brew").is_ok() {
                return Ok(PackageManager::Brew);
            }
            return Err(anyhow!("Homebrew not found. Please install it first: https://brew.sh"));
        }

        if which("apt-get").is_ok() {
            Ok(PackageManager::Apt)
        } else if which("yum").is_ok() {
            Ok(PackageManager::Yum)
        } else {
            Err(anyhow!("No supported package manager found. Please install apt-get or yum."))
        }
    }

    /// Get the recipe for this package manager
    pub fn recipe(&self) -> Recipe {
        match self {
            PackageManager::Apt => Recipe::new()
                .with_update_steps(vec![
                    ("sudo", vec!["apt-get", "-y", "clean"]),
                    ("sudo", vec!["apt-get", "-y", "update"]),
                ])
                .with_install_step("sudo", vec!["apt-get", "install", "-y"]),
            PackageManager::Yum => Recipe::new()
                .with_update_steps(vec![
                    ("sudo", vec!["yum", "-y", "clean", "all"]),
                    ("sudo", vec!["yum", "-y", "makecache"]),
                ])
                .with_install_step("sudo", vec!["yum", "install", "-y"]),
            PackageManager::Brew => Recipe::new()
                .with_update_steps(vec![("brew", vec!["update"])])
                .with_install_step("brew", vec!["install"]),
        }
    }

    /// Install packages using this package manager
    pub fn install(&self, packages: &[Package]) -> Result<()> {
        if packages.is_empty() {
            return Ok(());
        }

        // Split packages into batches of 10 to avoid command line length limits
        const BATCH_SIZE: usize = 10;
        for chunk in packages.chunks(BATCH_SIZE) {
            let recipe = self.recipe();
            recipe.execute(self, chunk)?;
        }
        Ok(())
    }
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageManager::Apt => write!(f, "apt-get"),
            PackageManager::Yum => write!(f, "yum"),
            PackageManager::Brew => write!(f, "brew"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_display() {
        assert_eq!(PackageManager::Apt.to_string(), "apt-get");
        assert_eq!(PackageManager::Yum.to_string(), "yum");
        assert_eq!(PackageManager::Brew.to_string(), "brew");
    }

    #[test]
    fn test_recipe_generation() {
        let apt_recipe = PackageManager::Apt.recipe();
        assert_eq!(apt_recipe.install_command.0, "sudo");
        assert_eq!(
            apt_recipe.install_command.1,
            vec!["apt-get", "install", "-y"]
        );

        let yum_recipe = PackageManager::Yum.recipe();
        assert_eq!(yum_recipe.install_command.0, "sudo");
        assert_eq!(yum_recipe.install_command.1, vec!["yum", "install", "-y"]);

        let brew_recipe = PackageManager::Brew.recipe();
        assert_eq!(brew_recipe.install_command.0, "brew");
        assert_eq!(brew_recipe.install_command.1, vec!["install"]);
    }
}
