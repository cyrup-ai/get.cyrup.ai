use anyhow::Result;
use dirs::home_dir;
use std::fs;

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
    let zshrc = home.join(".zshrc");
    
    // Append our config if it's not already there
    let current = fs::read_to_string(&zshrc).unwrap_or_default();
    if !current.contains("# Cyrup shell configuration") {
        fs::write(&zshrc, format!("{}\n{}", current, SHELL_CONFIG))?;
    }
    
    Ok(())
}
