use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;

use super::Package;
use crate::system::run_cmd;

const RYE_INSTALL_URL: &str = "https://rye-up.com/get";
const RUSTUP_INSTALL_URL: &str = "https://sh.rustup.rs";

/// Global catalog of common packages with their proper mappings
pub static CATALOG: Lazy<HashMap<&'static str, Package>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // System Core
    m.insert(
        "system-core",
        Package::new("curl").with_alternatives(vec![
            "apt:apt-transport-https ca-certificates software-properties-common curl wget"
                .to_string(),
            "yum:yum-utils ca-certificates curl wget".to_string(),
            "brew:curl wget coreutils gnu-sed".to_string(),
        ]),
    );

    // Build Tools
    m.insert(
        "build-essential",
        Package::new("build-essential").with_alternatives(vec![
            "apt:build-essential pkg-config".to_string(),
            "yum:gcc gcc-c++ make".to_string(),
            "brew:gcc make xcode-select".to_string(),
        ]),
    );

    // Python Base (for Rye)
    m.insert(
        "python-base",
        Package::new("build-essential").with_alternatives(vec![
            "apt:build-essential libssl-dev libffi-dev zlib1g-dev".to_string(),
            "yum:gcc gcc-c++ openssl-devel libffi-devel zlib-devel".to_string(),
            "brew:openssl@3 xz".to_string(),
        ]),
    );

    // Shell Tools
    m.insert(
        "shell-tools",
        Package::new("zsh").with_alternatives(vec![
            "apt:zsh tmux htop tree ncdu jq fzf bat exa".to_string(),
            "yum:zsh tmux htop tree ncdu jq fzf bat exa".to_string(),
            "brew:zsh tmux htop tree ncdu jq fzf bat exa".to_string(),
        ]),
    );

    // Network Tools
    m.insert(
        "network-tools",
        Package::new("net-tools").with_alternatives(vec![
            "apt:openvpn net-tools dnsutils iputils-ping traceroute mtr nmap".to_string(),
            "yum:openvpn net-tools bind-utils iputils traceroute mtr nmap".to_string(),
            "brew:openvpn iproute2mac bind-tools traceroute mtr nmap".to_string(),
        ]),
    );

    // Rust Base (for rustup)
    m.insert(
        "rust-base",
        Package::new("build-essential").with_alternatives(vec![
            "apt:build-essential pkg-config libssl-dev".to_string(),
            "yum:gcc gcc-c++ openssl-devel".to_string(),
            "brew:openssl@3".to_string(),
        ]),
    );

    // Core Development Tools
    m.insert(
        "dev-tools",
        Package::new("ripgrep").with_alternatives(vec![
            "apt:ripgrep fd-find exa bat git-lfs".to_string(),
            "yum:ripgrep fd-find exa bat git-lfs".to_string(),
            "brew:ripgrep fd exa bat git-lfs".to_string(),
        ]),
    );

    // Browser Tools
    m.insert(
        "browsers",
        Package::new("firefox").with_alternatives(vec![
            "apt:firefox firefox-geckodriver chromium-browser".to_string(),
            "yum:firefox geckodriver chromium".to_string(),
            "brew:firefox geckodriver chromium".to_string(),
        ]),
    );

    // Database Tools
    m.insert(
        "database",
        Package::new("postgresql").with_alternatives(vec![
            "apt:postgresql postgresql-contrib postgresql-client".to_string(),
            "yum:postgresql postgresql-devel postgresql-libs".to_string(),
            "brew:postgresql@14 pgcli".to_string(),
        ]),
    );

    m
});

/// Get a package from the catalog
pub fn get(name: &str) -> Option<Package> {
    CATALOG.get(name).cloned()
}

/// Get multiple packages from the catalog
pub fn get_many(names: &[&str]) -> Vec<Package> {
    names.iter().filter_map(|name| get(name)).collect()
}

/// Get base system packages required for all installations
pub fn get_base_packages() -> Vec<Package> {
    get_many(&["system-core", "build-essential"])
}

/// Get development environment packages
pub fn get_dev_packages() -> Vec<Package> {
    get_many(&[
        "python-base", // For Rye
        "rust-base",   // For rustup
        "shell-tools",
        "network-tools",
        "dev-tools",
        "browsers",
        "database",
    ])
}

/// Get list of cargo packages to install
pub fn get_cargo_packages() -> Vec<&'static str> {
    vec![
        "dioxus-cli",
        "hurl",
        "ruff",
        "pueue",
        "zellij",
        "zoxide",
        "wasm-pack",
        "taplo-cli",
        "cargo-watch",
        "cargo-edit",
        "cargo-update",
    ]
}

/// Setup Python environment using Rye and uv
pub fn setup_python() -> Result<()> {
    println!("Installing Rye for Python management...");

    // Download and run Rye installer
    run_cmd("curl", &["-sSf", RYE_INSTALL_URL, "-o", "rye-install.sh"])
        .context("Failed to download Rye installer")?;

    run_cmd("bash", &["rye-install.sh"]).context("Failed to run Rye installer")?;

    run_cmd("rm", &["rye-install.sh"])?;

    // Add Rye to PATH for this session
    let home = env::var("HOME").context("HOME not set")?;
    let path = env::var("PATH").context("PATH not set")?;
    env::set_var("PATH", format!("{}/{}:{}", home, ".rye/shims", path));

    // Install and configure uv
    println!("Installing uv package installer...");
    run_cmd("rye", &["install", "uv"]).context("Failed to install uv")?;

    run_cmd("rye", &["config", "set", "pip.use-uv", "true"])
        .context("Failed to configure Rye to use uv")?;

    Ok(())
}

/// Setup Rust environment using rustup
pub fn setup_rust() -> Result<()> {
    println!("Installing Rust toolchain via rustup...");

    // Download and run rustup installer
    run_cmd(
        "curl",
        &[
            "--proto",
            "'=https'",
            "--tlsv1.2",
            "-sSf",
            RUSTUP_INSTALL_URL,
            "-o",
            "rustup-init.sh",
        ],
    )
    .context("Failed to download rustup")?;

    run_cmd("sh", &["rustup-init.sh", "-y", "--no-modify-path"])
        .context("Failed to run rustup installer")?;

    run_cmd("rm", &["rustup-init.sh"])?;

    // Add cargo bin to PATH for this session
    let home = env::var("HOME").context("HOME not set")?;
    let path = env::var("PATH").context("PATH not set")?;
    env::set_var("PATH", format!("{}/{}:{}", home, ".cargo/bin", path));

    // Install core cargo packages
    for package in get_cargo_packages() {
        println!("Installing cargo package: {}", package);
        run_cmd("cargo", &["install", package])
            .with_context(|| format!("Failed to install {}", package))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::manager::PackageManager;

    #[test]
    fn test_core_packages() {
        let packages = get_core_packages();
        assert!(!packages.is_empty());

        // Verify build-essential mappings
        let build = packages
            .iter()
            .find(|p| p.name_for(&PackageManager::Apt) == "build-essential")
            .unwrap();
        assert_eq!(build.name_for(&PackageManager::Yum), "gcc gcc-c++ make");
    }

    #[test]
    fn test_dev_packages() {
        let packages = get_dev_packages();
        assert!(!packages.is_empty());

        // Verify rust-utils mappings
        let utils = packages
            .iter()
            .find(|p| p.name_for(&PackageManager::Apt).contains("ripgrep"))
            .unwrap();
        assert!(utils.name_for(&PackageManager::Brew).contains("ripgrep"));
    }
}
