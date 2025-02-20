use std::collections::HashMap;
use anyhow::{Result, anyhow};
use crate::package::manager::PackageManager;

#[derive(Debug, Clone)]
pub struct PackageSpec {
    pub name: String,
    pub manager_type: PackageManager,
}

impl PackageSpec {
    pub fn new(name: impl Into<String>, manager_type: PackageManager) -> Self {
        Self {
            name: name.into(),
            manager_type,
        }
    }
}

#[derive(Debug)]
pub struct Recipe {
    pub packages: HashMap<String, Vec<PackageSpec>>,
}

impl Recipe {
    pub fn new() -> Self {
        let mut packages = HashMap::new();
        
        // Ubuntu packages (apt)
        packages.insert("ubuntu".to_string(), vec![
            PackageSpec::new("zsh", PackageManager::Apt),
            PackageSpec::new("tmux", PackageManager::Apt),
            PackageSpec::new("htop", PackageManager::Apt),
        ]);

        // Amazon Linux packages (yum)
        packages.insert("amazon".to_string(), vec![
            PackageSpec::new("zsh", PackageManager::Yum),
            PackageSpec::new("tmux", PackageManager::Yum),
            PackageSpec::new("htop", PackageManager::Yum),
        ]);

        // macOS packages (brew)
        packages.insert("macos".to_string(), vec![
            PackageSpec::new("zsh", PackageManager::Brew),
            PackageSpec::new("tmux", PackageManager::Brew),
            PackageSpec::new("htop", PackageManager::Brew),
        ]);

        Self { packages }
    }

    pub fn get_packages(&self, platform: &str) -> Result<&Vec<PackageSpec>> {
        self.packages.get(platform)
            .ok_or_else(|| anyhow!("Unsupported platform: {}", platform))
    }

    pub fn validate(&self) -> Result<()> {
        for (platform, packages) in &self.packages {
            // Check for empty package lists
            if packages.is_empty() {
                return Err(anyhow!("Empty package list for platform: {}", platform));
            }

            // Check for consistent package manager types
            let expected_type = packages[0].manager_type.clone();
            for package in packages {
                if package.manager_type != expected_type {
                    return Err(anyhow!(
                        "Inconsistent package manager types in platform {}: expected {:?}, got {:?}",
                        platform, expected_type, package.manager_type
                    ));
                }

                // Check for invalid package names
                if package.name.contains(char::is_whitespace) || package.name.is_empty() {
                    return Err(anyhow!(
                        "Invalid package name in platform {}: '{}'",
                        platform, package.name
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DevToolsRecipe {
    pub packages: HashMap<String, Vec<PackageSpec>>,
}

impl DevToolsRecipe {
    pub fn new() -> Self {
        let mut packages = HashMap::new();
        
        // Ubuntu packages (apt)
        packages.insert("ubuntu".to_string(), vec![
            PackageSpec::new("ripgrep", PackageManager::Apt),
            PackageSpec::new("fd-find", PackageManager::Apt),
            PackageSpec::new("bat", PackageManager::Apt),
            PackageSpec::new("exa", PackageManager::Apt),
        ]);

        // Amazon Linux packages (yum)
        packages.insert("amazon".to_string(), vec![
            PackageSpec::new("ripgrep", PackageManager::Yum),
            // Note: fd and bat are not available in default yum repos
            PackageSpec::new("exa", PackageManager::Yum),
        ]);

        // macOS packages (brew)
        packages.insert("macos".to_string(), vec![
            PackageSpec::new("ripgrep", PackageManager::Brew),
            PackageSpec::new("fd", PackageManager::Brew),  // Note: fd-find is just fd on brew
            PackageSpec::new("bat", PackageManager::Brew),
            PackageSpec::new("exa", PackageManager::Brew),
        ]);

        Self { packages }
    }

    pub fn get_packages(&self, platform: &str) -> Result<&Vec<PackageSpec>> {
        self.packages.get(platform)
            .ok_or_else(|| anyhow!("Unsupported platform: {}", platform))
    }

    pub fn validate(&self) -> Result<()> {
        for (platform, packages) in &self.packages {
            if packages.is_empty() {
                return Err(anyhow!("Empty package list for platform: {}", platform));
            }

            let expected_type = packages[0].manager_type.clone();
            for package in packages {
                if package.manager_type != expected_type {
                    return Err(anyhow!(
                        "Inconsistent package manager types in platform {}: expected {:?}, got {:?}",
                        platform, expected_type, package.manager_type
                    ));
                }

                if package.name.contains(char::is_whitespace) || package.name.is_empty() {
                    return Err(anyhow!(
                        "Invalid package name in platform {}: '{}'",
                        platform, package.name
                    ));
                }
            }
        }
        Ok(())
    }
}
