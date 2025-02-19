use anyhow::Result;
use dirs::home_dir;
use std::{env, fs};

const SHELL_CONFIG: &str = r#"# Cyrup shell configuration
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_CACHE_HOME="$HOME/.cache"
export XDG_DATA_HOME="$HOME/.local/share"

# Rust
export CARGO_HOME="$XDG_DATA_HOME/cargo"
export RUSTUP_HOME="$XDG_DATA_HOME/rustup"

# Path
export PATH="$CARGO_HOME/bin:$PATH"

# Aliases
alias ls='ls --color=auto'
alias ll='ls -la'
alias grep='grep --color=auto'
"#;

pub fn setup() -> Result<()> {
    println!("🐚 Setting up shell environment");

    let home = home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    // Detect shell and get config file
    let shell = env::var("SHELL").unwrap_or_default();
    let config_file = match shell.as_str() {
        s if s.ends_with("/zsh") => home.join(".zshrc"),
        s if s.ends_with("/bash") => home.join(".bashrc"),
        _ => home.join(".profile"), // Fallback to .profile for other shells
    };

    println!("📝 Using shell config: {}", config_file.display());

    // Read existing config or create new one
    let current = if config_file.exists() {
        fs::read_to_string(&config_file)
            .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", config_file.display(), e))?
    } else {
        String::new()
    };

    // Only append our config if it's not already there
    if !current.contains("# Cyrup shell configuration") {
        let new_config = if current.is_empty() {
            SHELL_CONFIG.to_string()
        } else {
            format!("{}\n\n{}", current.trim_end(), SHELL_CONFIG)
        };

        fs::write(&config_file, new_config)
            .map_err(|e| anyhow::anyhow!("Failed to write {}: {}", config_file.display(), e))?;

        println!("✨ Added shell configuration");
    } else {
        println!("✓ Shell configuration already present");
    }

    Ok(())
}
