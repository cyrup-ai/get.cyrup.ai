use anyhow::{anyhow, Result, Context};
use crate::config::Config;
use crate::system::{self, PackageCache, RETRY_ATTEMPTS};
use std::fs;
use dirs::home_dir;

pub fn collect_env() -> Config {
    Config::new()
        .key("AWS_ACCESS_KEY_ID")
        .key("AWS_SECRET_ACCESS_KEY")
        .key("OPENAI_API_KEY")
        .key("ANTHROPIC_API_KEY")
        .key("MISTRAL_API_KEY")
        .key("HF_TOKEN")
        .key("HUGGINGFACE_HUB_TOKEN")
        .key("REPLICATE_API_KEY")
        .key("GEMINI_API_KEY")
        .key("GROQ_API_KEY")
        .key("PERPLEXITY_API_KEY")
        .key("ELEVEN_LABS_API_KEY")
        .key("FIREWORKS_API_KEY")
        .key("TAVILY_API_KEY")
        .key("SERPER_API_KEY")
        .key("NEO4J_URI")
        .key("NEO4J_USER")
        .key("NEO4J_PASSWORD")
        .key("QDRANT_HOST")
        .key("QDRANT_PORT")
        .key("SURREAL_HOST")
        .key("SURREAL_PORT")
        .key("SURREAL_USER")
        .key("SURREAL_PWD")
        .key("SURREAL_NS")
        .key("SURREAL_DB")
        .key("NGROK_AUTHTOKEN")
        .key("VULTR_API_KEY")
        .key("LIVEKIT_API_KEY")
        .key("LIVEKIT_API_SECRET")
        .key("BITWARDEN_API_KEY")
        .key("BITWARDEN_CLIENT_ID")
        .key("CLOUDINARY_API_KEY")
        .key("CLOUDINARY_API_SECRET")
        .key("TWILIO_ACCOUNT_SID")
        .key("TWILIO_AUTH_TOKEN")
        .key("SPOTIFY_CLIENT_ID")
        .key("SPOTIFY_CLIENT_SECRET")
}

pub async fn run_installer() -> Result<()> {
    let (os, arch) = system::detect_os_and_arch().context("Failed to detect OS and architecture")?;
    println!("Detected OS: {}, Architecture: {}", os, arch);

    // Ensure we have sudo access before proceeding
    system::ensure_sudo_access()?;

    ensure_home_is_set(&os)?;
    setup_package_manager(&os)?;
    update_system(&os)?;
    install_packages_for_platform(&os)?;
    setup_shell_environment()?;

    Ok(())
}

pub fn ensure_home_is_set(os: &str) -> Result<()> {
    if std::env::var("HOME").is_err() {
        let home_path = match os {
            "ubuntu" | "amazon" => "/root".to_string(),
            _ => format!("/Users/{}", whoami::username()),
        };
        std::env::set_var("HOME", &home_path);
    }
    Ok(())
}

pub fn setup_package_manager(os: &str) -> Result<()> {
    println!("Setting up package manager...");
    match os {
        "ubuntu" => {
            // Ensure apt is ready
            system::retry_with_backoff(|| system::run_cmd("sudo", &["-n", "apt-get", "-y", "clean"]), RETRY_ATTEMPTS)?;
            system::retry_with_backoff(|| system::run_cmd("sudo", &["-n", "apt-get", "-y", "update"]), RETRY_ATTEMPTS)?;
        }
        "amazon" => {
            // Clean yum cache and ensure repos are ready
            system::retry_with_backoff(|| system::run_cmd("sudo", &["-n", "yum", "-y", "clean", "all"]), RETRY_ATTEMPTS)?;
            system::retry_with_backoff(|| system::run_cmd("sudo", &["-n", "yum", "-y", "makecache"]), RETRY_ATTEMPTS)?;
        }
        "macos" => {
            if which::which("brew").is_err() {
                println!("Installing Homebrew...");
                system::run_cmd("bash", &["-c", "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""])?;

                if std::env::consts::ARCH == "aarch64" {
                    system::run_cmd("bash", &["-c", "eval \"$(/opt/homebrew/bin/brew shellenv)\""])?;
                }
            }
            system::retry_with_backoff(|| system::run_cmd("brew", &["update", "--force"]), RETRY_ATTEMPTS)?;
        }
        _ => return Err(anyhow!("Unsupported operating system: {os}")),
    }
    Ok(())
}

pub fn setup_shell_environment() -> Result<()> {
    println!("Setting up shell environment...");
    
    let home = home_dir().ok_or_else(|| anyhow!("Cannot determine HOME directory"))?;
    let zshrc = home.join(".zshrc");
    
    // Write shell configuration
    fs::write(&zshrc, include_str!("../config/shell/zshrc"))?;
    
    Ok(())
}
