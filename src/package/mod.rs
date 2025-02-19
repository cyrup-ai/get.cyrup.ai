pub mod catalog;
pub mod manager;
pub mod recipe;
pub mod sys;

// Re-export commonly used items
pub use catalog::*;
pub use manager::PackageManager;
pub use recipe::Recipe;
pub use sys::{detect_platform, ensure_sudo_access, PackageCache};

use anyhow::Result;
use std::fmt;

/// Represents a system package that can be installed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Package {
    /// Name of the package
    name: String,
    /// Version constraint (if any)
    version: Option<String>,
    /// Alternative names for different package managers
    alternatives: Vec<String>,
}

impl Package {
    /// Create a new package with a specific name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            alternatives: Vec::new(),
        }
    }

    /// Add a version constraint to the package
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Add alternative package names for different package managers
    pub fn with_alternatives(mut self, alternatives: Vec<String>) -> Self {
        self.alternatives = alternatives;
        self
    }

    /// Get the best package name for the given package manager
    pub fn name_for(&self, pm: &manager::PackageManager) -> &str {
        // Check if there's a specific alternative for this package manager
        match pm {
            manager::PackageManager::Apt => self
                .alternatives
                .iter()
                .find(|name| name.starts_with("apt:"))
                .map(|name| &name[4..])
                .unwrap_or(&self.name),
            manager::PackageManager::Yum => self
                .alternatives
                .iter()
                .find(|name| name.starts_with("yum:"))
                .map(|name| &name[4..])
                .unwrap_or(&self.name),
            manager::PackageManager::Brew => self
                .alternatives
                .iter()
                .find(|name| name.starts_with("brew:"))
                .map(|name| &name[5..])
                .unwrap_or(&self.name),
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.version {
            Some(version) => write!(f, "{}={}", self.name, version),
            None => write!(f, "{}", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use manager::PackageManager;

    #[test]
    fn test_package_creation() {
        let pkg = Package::new("git")
            .with_version("2.0.0")
            .with_alternatives(vec!["apt:git-all".to_string(), "brew:git".to_string()]);

        assert_eq!(pkg.name_for(&PackageManager::Apt), "git-all");
        assert_eq!(pkg.name_for(&PackageManager::Yum), "git");
        assert_eq!(pkg.name_for(&PackageManager::Brew), "git");
    }

    #[test]
    fn test_package_display() {
        let pkg = Package::new("git");
        assert_eq!(pkg.to_string(), "git");

        let pkg_with_version = Package::new("git").with_version("2.0.0");
        assert_eq!(pkg_with_version.to_string(), "git=2.0.0");
    }
}
