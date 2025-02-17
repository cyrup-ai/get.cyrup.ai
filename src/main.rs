use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cyrup_install::install().await
}

// ============================================================================
// Configuration Constants
// ============================================================================

// Tool Configuration Templates
const RUSTFMT_CONFIG: &str = r#"
max_width = 100
tab_spaces = 4
edition = "2021"
use_small_heuristics = "Max"
imports_granularity = "Module"
group_imports = "StdExternalCrate"
"#;

const CARGO_CONFIG: &str = r#"
[build]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/opt/llvm/bin/ld64.lld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld"]

[registries.crates-io]
protocol = "sparse"
"#;

const STARSHIP_CONFIG: &str = r#"
format = """
$username\
$hostname\
$directory\
$git_branch\
$git_state\
$git_status\
$cmd_duration\
$line_break\
$python\
$character"""

[directory]
style = "blue"

[character]
success_symbol = "[❯](purple)"
error_symbol = "[❯](red)"
vimcmd_symbol = "[❮](green)"

[git_branch]
format = "[$branch]($style)"
style = "bright-black"

[git_status]
format = "[[(*$conflicted$untracked$modified$staged$renamed$deleted)](218) ($ahead_behind$stashed)]($style)"
style = "cyan"
conflicted = "​"
untracked = "​"
modified = "​"
staged = "​"
renamed = "​"
deleted = "​"
stashed = "≡"

[git_state]
format = '\([$state( $progress_current/$progress_total)]($style)\) '
style = "bright-black"

[cmd_duration]
format = "[$duration]($style) "
style = "yellow"

[python]
format = "[$virtualenv]($style) "
style = "bright-black"
"#;

const ALACRITTY_CONFIG: &str = r#"
window:
  padding:
    x: 5
    y: 5
  dynamic_padding: true
  decorations: none
  startup_mode: Maximized
  option_as_alt: Both

font:
  normal:
    family: "FiraCode Nerd Font"
    style: Regular
  size: 14.0

colors:
  primary:
    background: '#1a1b26'
    foreground: '#c0caf5'
"#;

const TMUX_CONFIG: &str = r#"
# Enable mouse support
set -g mouse on

# Start windows and panes at 1, not 0
set -g base-index 1
setw -g pane-base-index 1

# Use 256 colors
set -g default-terminal "screen-256color"
set -ga terminal-overrides ",*256col*:Tc"

# Increase scrollback buffer size
set -g history-limit 50000

# Status bar
set -g status-style 'bg=#1a1b26 fg=#c0caf5'

# Vim mode
set-window-option -g mode-keys vi

# Smart pane switching with awareness of Vim splits
bind -n C-h run "(tmux display-message -p '#{pane_current_command}' | grep -iq vim && tmux send-keys C-h) || tmux select-pane -L"
bind -n C-j run "(tmux display-message -p '#{pane_current_command}' | grep -iq vim && tmux send-keys C-j) || tmux select-pane -D"
bind -n C-k run "(tmux display-message -p '#{pane_current_command}' | grep -iq vim && tmux send-keys C-k) || tmux select-pane -U"
bind -n C-l run "(tmux display-message -p '#{pane_current_command}' | grep -iq vim && tmux send-keys C-l) || tmux select-pane -R"
"#;

const GITCONFIG: &str = r#"
[core]
    editor = hx
    pager = delta

[interactive]
    diffFilter = delta --color-only

[delta]
    navigate = true
    light = false
    side-by-side = true
"#;

// Software Recipe Definition
#[derive(Debug)]
enum PlatformSupport {
    Supported(&'static [&'static str]),
    Unsupported(&'static str), // Reason why it's not supported
}

struct Recipe {
    name: &'static str,
    description: &'static str,
    platforms: RecipePlatforms,
    required: bool,
}

struct RecipePlatforms {
    ubuntu: PlatformSupport,
    amazon: PlatformSupport,
    macos: PlatformSupport,
}

// Core Software Recipes
const RECIPES: &[Recipe] = &[
    Recipe {
        name: "system-core",
        description: "Core system package manager and OS updates",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&[
                "apt-transport-https",
                "ca-certificates",
                "software-properties-common"
            ]),
            amazon: PlatformSupport::Supported(&[
                "yum-utils",
                "ca-certificates"
            ]),
            macos: PlatformSupport::Supported(&[
                "curl"  // Required for Homebrew installation
            ]),
        },
        required: true,
    },
    Recipe {
        name: "build-tools",
        description: "Core build tools and compilers",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&["build-essential", "gcc", "g++", "make"]),
            amazon: PlatformSupport::Supported(&["gcc", "gcc-c++", "make"]),
            macos: PlatformSupport::Supported(&["gcc"]),
        },
        required: true,
    },
    Recipe {
        name: "python-dev",
        description: "Python development environment",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&[
                "python3",
                "python3-pip",
                "python3-venv",
                "python3-dev"
            ]),
            amazon: PlatformSupport::Supported(&[
                "python3",
                "python3-pip",
                "python3-devel"
            ]),
            macos: PlatformSupport::Supported(&["python3"]),
        },
        required: true,
    },
    Recipe {
        name: "shell-tools",
        description: "Shell and terminal utilities",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&["zsh", "tmux", "htop"]),
            amazon: PlatformSupport::Supported(&["zsh", "tmux", "htop"]),
            macos: PlatformSupport::Supported(&["zsh", "tmux", "htop"]),
        },
        required: true,
    },
    Recipe {
        name: "network-tools",
        description: "Networking utilities",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&[
                "openvpn",
                "net-tools",
                "dnsutils",
                "iputils-ping"
            ]),
            amazon: PlatformSupport::Supported(&[
                "openvpn",
                "net-tools",
                "bind-utils",
                "iputils"
            ]),
            macos: PlatformSupport::Supported(&["openvpn"]),
        },
        required: true,
    },
    Recipe {
        name: "rust",
        description: "Rust programming language and toolchain",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&["curl", "build-essential"]),
            amazon: PlatformSupport::Supported(&["curl", "gcc", "gcc-c++"]),
            macos: PlatformSupport::Supported(&["curl"]),
        },
        required: true,
    },
    Recipe {
        name: "rust-tools",
        description: "Essential Rust-based development tools",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&[
                "rg", "fd-find", "starship", "nu", "alacritty",
                "dioxus", "hurl", "bun", "ruff", "polars",
                "pueue", "rust-analyzer", "zellij", "zoxide",
                "wasm-pack", "tree-sitter-cli", "taplo",
                "minijinja-cli", "jinja-lsp", "chrome",
                "firefox", "geckodriver", "sniffnet",
                "smartdns-rs", "surreal", "wiki"
            ]),
            amazon: PlatformSupport::Supported(&[
                "rg", "fd-find", "starship", "nu", "alacritty",
                "dioxus", "hurl", "bun", "ruff", "polars",
                "pueue", "rust-analyzer", "zellij", "zoxide"
            ]),
            macos: PlatformSupport::Supported(&["rg", "fd-find", "starship", "nu", "alacritty"]),
        },
        required: false,
    },
    Recipe {
        name: "warp-terminal",
        description: "Modern Rust-based terminal",
        platforms: RecipePlatforms {
            ubuntu: PlatformSupport::Supported(&["warp-terminal"]),
            amazon: PlatformSupport::Unsupported(
                "No official RPM package available yet. Consider using alternative terminal."
            ),
            macos: PlatformSupport::Supported(&["warp"]),
        },
        required: false,
    },
];

impl Recipe {
    fn get_packages(&self, os: &str) -> Vec<&'static str> {
        match os {
            "ubuntu" => match &self.platforms.ubuntu {
                PlatformSupport::Supported(pkgs) => {
                    println!("🔧 Installing {} - {}", self.name, self.description);
                    pkgs.to_vec()
                }
                PlatformSupport::Unsupported(reason) => {
                    if self.required {
                        println!("⚠️  Required software {} is not supported on Ubuntu: {}", self.name, reason);
                    } else {
                        println!("ℹ️  Optional software {} is not supported on Ubuntu: {}", self.name, reason);
                    }
                    vec![]
                }
            },
            "amazon" => match &self.platforms.amazon {
                PlatformSupport::Supported(pkgs) => {
                    println!("🔧 Installing {} - {}", self.name, self.description);
                    pkgs.to_vec()
                }
                PlatformSupport::Unsupported(reason) => {
                    if self.required {
                        println!("⚠️  Required software {} is not supported on Amazon Linux: {}", self.name, reason);
                    } else {
                        println!("ℹ️  Optional software {} is not supported on Amazon Linux: {}", self.name, reason);
                    }
                    vec![]
                }
            },
            "macos" => match &self.platforms.macos {
                PlatformSupport::Supported(pkgs) => {
                    println!("🔧 Installing {} - {}", self.name, self.description);
                    pkgs.to_vec()
                }
                PlatformSupport::Unsupported(reason) => {
                    if self.required {
                        println!("⚠️  Required software {} is not supported on macOS: {}", self.name, reason);
                    } else {
                        println!("ℹ️  Optional software {} is not supported on macOS: {}", self.name, reason);
                    }
                    vec![]
                }
            },
            _ => vec![],
        }
    }
}

// Installation Settings
const RETRY_ATTEMPTS: u32 = 3;
const CACHE_DURATION_SECS: u64 = 24 * 60 * 60; // 24 hours

// URLs and Paths
const HOMEBREW_INSTALL_URL: &str = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";
const WARP_DEB_URL: &str = "https://releases.warp.dev/stable/v0.2024.02.20.08.01.stable_02/warp-terminal-x86_64.deb";
const WARP_RPM_URL: &str = "https://releases.warp.dev/stable/v0.2024.02.20.08.01.stable_02/warp-terminal-x86_64.rpm";

const UBUNTU_WARP_DEPS: &[&str] = &["libwebkit2gtk-4.0-37", "libgtk-3-0"];
const AMAZON_WARP_DEPS: &[&str] = &["webkit2gtk3", "gtk3"];

// ZSH configuration template
const ZSH_CONFIG_TEMPLATE: &str = r#"# Path configuration
{PATH_EXPORTS}

# Antigen: zsh plugin manager
source "$XDG_DATA_HOME/antigen/antigen.zsh"

# XDG Base Directory
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_CACHE_HOME="$HOME/.cache"
export XDG_DATA_HOME="$HOME/.local/share"

# Development paths
export CARGO_HOME="$HOME/.cargo"
export RUSTUP_HOME="$HOME/.rustup"
export GOPATH="$HOME/.local/share/go"
export GOBIN="$HOME/.local/bin"

# Development directories
export CYRUP_HOME="$HOME/cyrup"
export FORGE_HOME="$HOME/forge"

# Tool-specific XDG compliance
export LESSHISTFILE="$XDG_CACHE_HOME/less/history"
export INPUTRC="$XDG_CONFIG_HOME/readline/inputrc"
export CARGO_HOME="$XDG_DATA_HOME/cargo"

# Specify your plugins here
antigen bundle zsh-users/zsh-completions
antigen bundle zsh-users/zsh-syntax-highlighting
antigen bundle git

# Apply the plugin configurations
antigen apply

# Aliases
{ALIASES}
"#;

struct PackageCache {
    cache_dir: std::path::PathBuf,
}

impl PackageCache {
    fn new() -> Result<Self> {
        let cache_dir = home_dir()
            .ok_or_else(|| anyhow!("Cannot determine home directory"))?
            .join(".cache/cypackages");
        fs::create_dir_all(&cache_dir)?;
        Ok(Self { cache_dir })
    }

    fn get_cache_path(&self, os: &str, package: &str) -> std::path::PathBuf {
        self.cache_dir.join(format!("{}_{}", os, package))
    }

    fn is_fresh(&self, os: &str, package: &str) -> bool {
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

    fn update(&self, os: &str, package: &str) -> Result<()> {
        let cache_path = self.get_cache_path(os, package);
        fs::write(cache_path, SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
            .to_string())?;
        Ok(())
    }
}

fn retry_with_backoff<F, T>(mut f: F, retries: u32) -> Result<T>
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

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting system setup with recipe-based configuration...");
    run_installer().await?;
    println!("Installation completed successfully!");
    Ok(())
}

fn ensure_sudo_access() -> Result<()> {
    println!("Checking sudo access...");
    
    // Check if we're already root
    if whoami::username() == "root" {
        return Ok(());
    }

    // Check if sudo is available
    if which("sudo").is_err() {
        return Err(anyhow!("sudo is not installed. Please run as root or install sudo."));
    }

    // Try to cache sudo credentials and validate access
    match run_cmd_output("sudo", &["-n", "true"]) {
        Ok(_) => println!("Sudo access already cached"),
        Err(_) => {
            println!("Please enter your sudo password (will be cached for script duration)");
            run_cmd("sudo", &["-v"])?;
            // Keep sudo timestamp fresh in background
            run_cmd("bash", &["-c", "while true; do sudo -n true; sleep 50; kill -0 $$ || exit; done 2>/dev/null &"])?;
        }
    }
    Ok(())
}

async fn run_installer() -> Result<()> {
    let (os, arch) = detect_os_and_arch().context("Failed to detect OS and architecture")?;
    println!("Detected OS: {}, Architecture: {}", os, arch);

    // Ensure we have sudo access before proceeding
    ensure_sudo_access()?;

    ensure_home_is_set(&os)?;
    setup_package_manager(&os)?;  // Ensure package manager is ready
    update_system(&os)?;
    install_packages_for_platform(&os)?;
    setup_rust_environment(&os)?;  // Install Rust after its dependencies
    setup_shell_environment()?;
    install_fonts(&os)?;

    Ok(())
}

fn detect_os_and_arch() -> Result<(String, String)> {
    let raw_os = std::env::consts::OS;
    let arch = std::env::consts::ARCH.to_string();

    match raw_os {
        "macos" => Ok(("macos".to_string(), arch)),
        "linux" => {
            let distro = parse_linux_distro_id().unwrap_or_else(|_| "ubuntu".to_string());
            Ok((distro, arch))
        }
        _ => Err(anyhow!("Unsupported operating system: {}", raw_os)),
    }
}

fn parse_linux_distro_id() -> Result<String> {
    let content = fs::read_to_string("/etc/os-release")
        .context("Could not read /etc/os-release")?;
    for line in content.lines() {
        if line.starts_with("ID=") {
            let id_line = line.trim_start_matches("ID=").replace('"', "");
            if id_line.contains("amzn") || id_line.contains("amazon") {
                return Ok("amazon".to_string());
            } else if id_line.contains("ubuntu") {
                return Ok("ubuntu".to_string());
            }
        }
    }
    Ok("ubuntu".to_string())
}

fn ensure_home_is_set(os: &str) -> Result<()> {
    if env::var("HOME").is_err() {
        let home_path = match os {
            "ubuntu" | "amazon" => "/root".to_string(),
            _ => format!("/Users/{}", whoami::username()),
        };
        env::set_var("HOME", &home_path);
    }
    Ok(())
}

fn setup_package_manager(os: &str) -> Result<()> {
    println!("Setting up package manager...");
    match os {
        "ubuntu" => {
            // Ensure apt is ready
            retry_with_backoff(|| run_cmd("sudo", &["-n", "apt-get", "-y", "clean"]), RETRY_ATTEMPTS)?;
            retry_with_backoff(|| run_cmd("sudo", &["-n", "apt-get", "-y", "update"]), RETRY_ATTEMPTS)?;
        }
        "amazon" => {
            // Clean yum cache and ensure repos are ready
            retry_with_backoff(|| run_cmd("sudo", &["-n", "yum", "-y", "clean", "all"]), RETRY_ATTEMPTS)?;
            retry_with_backoff(|| run_cmd("sudo", &["-n", "yum", "-y", "makecache"]), RETRY_ATTEMPTS)?;
        }
        "macos" => {
            if which("brew").is_err() {
                println!("Installing Homebrew...");
                run_cmd("bash", &["-c", &format!("/bin/bash -c \"$(curl -fsSL {})\"", HOMEBREW_INSTALL_URL)])?;
                println!("Homebrew installed successfully");

                if std::env::consts::ARCH == "aarch64" {
                    run_cmd("bash", &["-c", "eval \"$(/opt/homebrew/bin/brew shellenv)\""])?;
                }
            }
            retry_with_backoff(|| run_cmd("brew", &["update", "--force"]), RETRY_ATTEMPTS)?;
        }
        _ => return Err(anyhow!("Unsupported operating system: {os}")),
    }
    Ok(())
}

fn update_system(os: &str) -> Result<()> {
    println!("Updating system packages...");
    match os {
        "ubuntu" => {
            retry_with_backoff(|| run_cmd("sudo", &["-n", "apt-get", "-y", "update"]), RETRY_ATTEMPTS)?;
            retry_with_backoff(|| run_cmd("sudo", &["-n", "apt-get", "-y", "upgrade"]), RETRY_ATTEMPTS)?;
            retry_with_backoff(|| run_cmd("sudo", &["-n", "apt-get", "-y", "dist-upgrade"]), RETRY_ATTEMPTS)?;
        }
        "amazon" => {
            retry_with_backoff(|| run_cmd("sudo", &["-n", "yum", "-y", "update"]), RETRY_ATTEMPTS)?;
            retry_with_backoff(|| run_cmd("sudo", &["-n", "yum", "-y", "upgrade"]), RETRY_ATTEMPTS)?;
        }
        "macos" => {
            // Homebrew updates
            retry_with_backoff(|| run_cmd("brew", &["upgrade"]), RETRY_ATTEMPTS)?;
            
            // System and App Store updates
            println!("Checking for macOS system and App Store updates...");
            
            // List available updates first
            if let Ok(output) = run_cmd_output("sudo", &["-n", "softwareupdate", "--list"]) {
                if String::from_utf8_lossy(&output.stdout).contains("No new software available") {
                    println!("System is up to date");
                } else {
                    println!("Installing system updates (this may take a while)...");
                    retry_with_backoff(|| run_cmd("sudo", &["-n", "softwareupdate", "--install", "--all", "--agree-to-license"]), RETRY_ATTEMPTS)
                        .context("Failed to run system updates")?;
                }
            }
        }
        _ => return Err(anyhow!("Unsupported operating system: {os}")),
    }
    Ok(())
}

fn install_packages_for_platform(os: &str) -> Result<()> {
    let cache = PackageCache::new()
        .context("Failed to initialize package cache")?;

    println!("Processing installation recipes...");
    let mut installed_count = 0;

    // Setup package manager if needed
    if os == "macos" && which("brew").is_err() {
        println!("Installing Homebrew...");
        run_cmd("bash", &["-c", &format!("/bin/bash -c \"$(curl -fsSL {})\"", HOMEBREW_INSTALL_URL)])?;
        println!("Homebrew installed successfully");

        if std::env::consts::ARCH == "aarch64" {
            run_cmd("bash", &["-c", "eval \"$(/opt/homebrew/bin/brew shellenv)\""])?;
        }
    }

    // Process each recipe
    for recipe in RECIPES {
        let packages = recipe.get_packages(os);
        for &package in &packages {
            if !cache.is_fresh(os, package) {
                if !check_package_installed(os, package) {
                    println!("Installing package: {}", package);
                    match os {
                        "ubuntu" => {
                            retry_with_backoff(|| run_cmd("sudo", &["-n", "DEBIAN_FRONTEND=noninteractive", "apt-get", "install", "-y", "--no-install-recommends", package]), RETRY_ATTEMPTS)
                                .with_context(|| format!("Failed to install {}", package))?;
                        }
                        "amazon" => {
                            retry_with_backoff(|| run_cmd("sudo", &["-n", "yum", "install", "-y", package]), RETRY_ATTEMPTS)
                                .with_context(|| format!("Failed to install {}", package))?;
                        }
                        "macos" => {
                            retry_with_backoff(|| run_cmd("brew", &["install", "--force", package]), RETRY_ATTEMPTS)
                                .with_context(|| format!("Failed to install {}", package))?;
                        }
                        _ => {}
                    }
                    installed_count += 1;
                }
                cache.update(os, package)?;
            }
        }
    }

    // Handle special cases
    match os {
        "ubuntu" => {
            // Handle Warp installation for Ubuntu
            if let Some(recipe) = RECIPES.iter().find(|r| r.name == "warp-terminal") {
                let packages = recipe.get_packages(os);
                if !packages.is_empty() && !check_package_installed(os, "warp-terminal") {
                    println!("Installing Warp terminal...");
                    let temp_dir = env::temp_dir();
                    let deb_path = temp_dir.join("warp.deb");
                    
                    retry_with_backoff(|| {
                        run_cmd("wget", &[
                            WARP_DEB_URL,
                            "-O",
                            deb_path.to_str().unwrap()
                        ])
                    }, RETRY_ATTEMPTS).context("Failed to download Warp")?;

                    retry_with_backoff(|| run_cmd("sudo", &["apt-get", "install", "-y"].iter().chain(UBUNTU_WARP_DEPS).copied().collect::<Vec<_>>().as_slice()), RETRY_ATTEMPTS)
                        .context("Failed to install Warp dependencies")?;

                    run_cmd("sudo", &["dpkg", "-i", deb_path.to_str().unwrap()])?;
                    run_cmd("sudo", &["apt-get", "install", "-f"])?;
                    fs::remove_file(deb_path)?;
                    cache.update(os, "warp-terminal")?;
                    installed_count += 1;
                }
            }
        }
        "amazon" => {
            // Install EPEL if needed
            if !cache.is_fresh(os, "epel-release") && !check_package_installed(os, "epel-release") {
                println!("Installing EPEL repository...");
                retry_with_backoff(|| run_cmd("sudo", &["amazon-linux-extras", "install", "-y", "epel"]), RETRY_ATTEMPTS)
                    .context("Failed to install EPEL")?;
                cache.update(os, "epel-release")?;
                installed_count += 1;
            }
        }
        _ => {}
    }

    println!("Successfully installed {} new package(s)", installed_count);
    println!("{} package installation complete", os);

    Ok(())
}

fn setup_shell_environment() -> Result<()> {
    println!("Setting up shell environment...");
    
    let home = home_dir().ok_or_else(|| anyhow!("Cannot determine HOME directory"))?;
    let zshrc_path = home.join(".zshrc");
    
    // Don't modify existing ZSH config
    if zshrc_path.exists() {
        println!("Existing ZSH configuration found, skipping shell setup...");
        return Ok(());
    }
    // Generate ZSH config
    let path_exports = vec![
        "$HOME/.local/bin",
        "$HOME/.cargo/bin",
        "$HOME/.local/share/bob/nvim-bin",
    ].iter()
        .map(|&path| format!("export PATH=\"{}:$PATH\"", path))
        .collect::<Vec<_>>()
        .join("\n");

    let aliases = vec![
        ("ll", "ls -la"),
        ("vim", "hx"),
        ("vi", "hx"),
        ("g", "git"),
    ].iter()
