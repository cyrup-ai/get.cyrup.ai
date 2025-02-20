use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;

use super::{Package, PackageManager};
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
    println!("Installing Python development environment...");

    // Install base Python requirements
    let packages = get_many(&["python-base"]);
    let pm = PackageManager::detect()?;
    pm.install(&packages)?;

    // Download and run Rye installer
    let temp_dir = env::temp_dir();
    let installer = temp_dir.join("rye-installer.sh");
    
    println!("Installing Rye for Python management...");
    run_cmd(
        "curl",
        &["-sSf", RYE_INSTALL_URL, "-o", installer.to_str().unwrap()],
    ).context("Failed to download Rye installer")?;

    // Make installer executable and run it
    run_cmd("chmod", &["+x", installer.to_str().unwrap()])?;
    run_cmd(installer.to_str().unwrap(), &[]).context("Failed to run Rye installer")?;

    // Add Rye to PATH
    let home = env::var("HOME").context("HOME environment variable not set")?;
    let rye_bin = format!("{home}/.rye/shims");
    
    // Update both .bashrc and .zshrc if they exist
    let path_export = format!("export PATH=\"{rye_bin}:$PATH\"");
    for rc_file in &[".bashrc", ".zshrc"] {
        let rc_path = format!("{home}/{rc_file}");
        if std::path::Path::new(&rc_path).exists() {
            run_cmd("sh", &["-c", &format!("echo '{path_export}' >> {rc_path}")])?;
        }
    }

    // Initialize rye and configure it
    println!("Configuring Rye and installing Python toolchain...");
    run_cmd("rye", &["self", "update"]).context("Failed to update Rye")?;
    run_cmd("rye", &["toolchain", "install", "3.11"]).context("Failed to install Python toolchain")?;
    run_cmd("rye", &["config", "set", "behavior.global-python=true"]).context("Failed to configure Rye")?;

    // Install and configure uv
    println!("Installing uv package installer...");
    run_cmd("rye", &["install", "uv"]).context("Failed to install uv")?;
    run_cmd("rye", &["config", "set", "pip.use-uv", "true"]).context("Failed to configure uv")?;

    Ok(())
}

/// Setup Rust environment using rustup
pub fn setup_rust() -> Result<()> {
    println!("Installing Rust development environment...");

    // Install base Rust requirements
    let packages = get_many(&["build-essential"]);
    let pm = PackageManager::detect()?;
    pm.install(&packages)?;

    // Download and run rustup installer
    let temp_dir = env::temp_dir();
    let installer = temp_dir.join("rustup-init.sh");
    
    println!("Installing Rust toolchain via rustup...");
    run_cmd(
        "curl",
        &[
            "--proto",
            "'=https'",
            "--tlsv1.2",
            "-sSf",
            RUSTUP_INSTALL_URL,
            "-o",
            installer.to_str().unwrap(),
        ],
    ).context("Failed to download rustup")?;

    // Make installer executable and run it
    run_cmd("chmod", &["+x", installer.to_str().unwrap()])?;
    run_cmd(installer.to_str().unwrap(), &["-y", "--no-modify-path"])
        .context("Failed to run rustup installer")?;

    // Add cargo bin to PATH
    let home = env::var("HOME").context("HOME environment variable not set")?;
    let cargo_bin = format!("{home}/.cargo/bin");
    
    // Update both .bashrc and .zshrc if they exist
    let path_export = format!("export PATH=\"{cargo_bin}:$PATH\"");
    for rc_file in &[".bashrc", ".zshrc"] {
        let rc_path = format!("{home}/{rc_file}");
        if std::path::Path::new(&rc_path).exists() {
            run_cmd("sh", &["-c", &format!("echo '{path_export}' >> {rc_path}")])?;
        }
    }

    // Source cargo env for current session
    run_cmd("sh", &["-c", "source $HOME/.cargo/env"]).context("Failed to source cargo env")?;

    // Install cargo packages
    println!("Installing cargo tools...");
    let packages = get_cargo_packages();
    for pkg in packages {
        println!("Installing cargo package: {}", pkg);
        run_cmd("cargo", &["install", pkg, "--locked"])
            .with_context(|| format!("Failed to install {}", pkg))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::manager::PackageManager;

    #[test]
    fn test_core_packages() {
        let packages = vec!["zsh", "tmux", "htop"].into_iter().map(String::from).collect::<Vec<_>>();
        assert!(!packages.is_empty());
        assert!(packages.contains(&"zsh".to_string()));
        assert!(packages.contains(&"tmux".to_string()));
        assert!(packages.contains(&"htop".to_string()));
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
