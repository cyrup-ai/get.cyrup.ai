mod recipes;
mod system;
mod shell;
mod gpu;

use anyhow::Result;

pub async fn install() -> Result<()> {
    println!("🚀 Cyrup AI Installer");
    println!("Installing everything you need to get started...");

    system::ensure_sudo_access()?;
    let (os, arch) = system::detect_platform()?;
    
    system::setup(&os)?;
    recipes::install_all(&os)?;
    shell::setup()?;
    gpu::detect_and_setup()?;

    println!("\n✨ Installation complete! You're ready to go!");
    Ok(())
}
