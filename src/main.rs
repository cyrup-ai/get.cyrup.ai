use anyhow::Result;

mod package;
mod recipes;
mod setcyrup;
mod shell;
mod gpu;
mod ui;
mod menu;
mod system;

#[tokio::main]
async fn main() -> Result<()> {
    // Show module selection menu and get installation plan
    let install_plan = menu::show_module_menu().await?;
    
    // Show confirmation with plan details
    if !menu::confirm_installation(&install_plan)? {
        println!("Installation cancelled. Exiting...");
        return Ok(());
    }

    // Create and initialize the UI
    let mut installer = ui::Installer::new()?;
    installer.start()?;

    // Run the installer
    let result = setcyrup::run_installer(&mut installer).await;

    // Show final status
    match result {
        Ok(_) => {
            installer.update_status("✨ Installation complete! Please restart your terminal.")?;
            std::process::exit(0);
        }
        Err(e) => {
            installer.update_status(format!("❌ Installation failed: {}", e))?;
            std::process::exit(1);
        }
    }
}
