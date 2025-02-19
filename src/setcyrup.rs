use anyhow::Result;
use crate::ui::Installer;
use crate::system;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

const RETRY_ATTEMPTS: u32 = 3;

pub async fn run_installer(installer: &mut Installer) -> Result<()> {
    installer.update_status("Starting installation...")?;
    installer.update_progress(0.1)?;

    // Create necessary directories
    let home = home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home.join(".config/cyrup");
    fs::create_dir_all(&config_dir)?;

    installer.update_status("Configuring shell...")?;
    installer.update_progress(0.3)?;

    // Configure shell
    configure_shell()?;

    installer.update_status("Installation complete!")?;
    installer.update_progress(1.0)?;

    Ok(())
}

fn configure_shell() -> Result<()> {
    let home = home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home.join(".config/cyrup");

    // Create shell config files
    let shell_dir = config_dir.join("shell");
    fs::create_dir_all(&shell_dir)?;

    // Write shell configs
    write_shell_config(&shell_dir)?;

    // Update user's shell config files
    update_user_shell_configs(&shell_dir)?;

    Ok(())
}

fn write_shell_config(shell_dir: &PathBuf) -> Result<()> {
    // Write zshrc
    let zshrc = shell_dir.join("zshrc");
    fs::write(&zshrc, "# Cyrup zsh configuration\nexport PATH=\"$HOME/.cargo/bin:$PATH\"\n")?;

    // Write bashrc
    let bashrc = shell_dir.join("bashrc");
    fs::write(&bashrc, "# Cyrup bash configuration\nexport PATH=\"$HOME/.cargo/bin:$PATH\"\n")?;

    Ok(())
}

fn update_user_shell_configs(shell_dir: &PathBuf) -> Result<()> {
    let home = home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    // Update .zshrc if it exists
    let user_zshrc = home.join(".zshrc");
    if user_zshrc.exists() {
        let source_line = format!("\n# Cyrup\nsource {}/zshrc\n", shell_dir.display());
        append_to_file(&user_zshrc, &source_line)?;
    }

    // Update .bashrc if it exists
    let user_bashrc = home.join(".bashrc");
    if user_bashrc.exists() {
        let source_line = format!("\n# Cyrup\nsource {}/bashrc\n", shell_dir.display());
        append_to_file(&user_bashrc, &source_line)?;
    }

    Ok(())
}

fn append_to_file(file: &PathBuf, content: &str) -> Result<()> {
    let current_content = fs::read_to_string(file)?;
    if !current_content.contains(content) {
        fs::write(file, format!("{}{}", current_content, content))?;
    }
    Ok(())
}
