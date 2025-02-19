use anyhow::{anyhow, Result};
use inquire::{MultiSelect, Select};
use std::fmt;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(Debug, Clone, PartialEq)]
pub enum InstallTarget {
    Host,
    DevContainer,
}

impl fmt::Display for InstallTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallTarget::Host => write!(f, "Install on Host System"),
            InstallTarget::DevContainer => write!(f, "Install in Dev Container"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstallModule {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub commands: Vec<String>,
}

impl fmt::Display for InstallModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct InstallPlan {
    pub target: Option<InstallTarget>,
    pub components: Vec<InstallModule>,
}

fn get_available_components() -> Vec<InstallModule> {
    vec![
        InstallModule {
            name: "Secret.Rust (cysec)".to_string(),
            description: "Secure Rust development environment with cryptographic tools and security features.".to_string(),
            dependencies: vec![
                "libssl-dev".to_string(),
                "pkg-config".to_string(),
            ],
            commands: vec![],
        },
        InstallModule {
            name: "Cyrup ML".to_string(),
            description: "Machine Learning environment with CUDA support, Python ML libraries, and Jupyter.".to_string(),
            dependencies: vec![
                "python3-pip".to_string(),
                "nvidia-cuda-toolkit".to_string(),
            ],
            commands: vec![],
        },
        InstallModule {
            name: "Cyrup AI (cyrup, cyrupd)".to_string(),
            description: "AI development environment with Cyrup CLI and daemon for local AI development.".to_string(),
            dependencies: vec![
                "python3-venv".to_string(),
                "python3-dev".to_string(),
            ],
            commands: vec![],
        },
        InstallModule {
            name: "───────────────────".to_string(), // Separator
            description: "".to_string(),
            dependencies: vec![],
            commands: vec![],
        },
        InstallModule {
            name: "Cyrup Developer".to_string(),
            description: "Development environment with Rust, Python, and common development tools.".to_string(),
            dependencies: vec![
                "build-essential".to_string(),
                "git".to_string(),
                "curl".to_string(),
            ],
            commands: vec![],
        },
    ]
}

fn get_target_description(target: &InstallTarget) -> String {
    match target {
        InstallTarget::Host => "Install directly on your host system.".to_string(),
        InstallTarget::DevContainer => "Install in a Docker-based VS Code Dev Container for isolated development.".to_string(),
    }
}

fn print_prompt() -> Result<()> {
    let mut stdout = std::io::stdout();
    stdout.execute(SetForegroundColor(Color::Cyan))?;
    stdout.execute(Print("❯ "))?;
    stdout.execute(SetForegroundColor(Color::White))?;
    stdout.execute(Print("cyrup"))?;
    stdout.execute(Print("\n\n"))?;
    stdout.execute(ResetColor)?;
    Ok(())
}

pub async fn show_module_menu() -> Result<InstallPlan> {
    print_prompt()?;

    // First select components
    let components = get_available_components();
    let selected_components = MultiSelect::new("Select components to install:", components.clone())
        .with_help_message("↑↓ to move, space to select, enter to confirm")
        .with_formatter(&|items| {
            let count = items.iter()
                .filter(|item| !item.value.name.starts_with("─"))
                .count();
            if count == 0 {
                "No components selected".to_string()
            } else {
                format!("{} components selected", count)
            }
        })
        .with_default(&[0]) // Default select Secret.rust which is first in the list
        .prompt()?;

    // Filter out any separator that might have been selected
    let selected_components: Vec<InstallModule> = selected_components
        .into_iter()
        .filter(|m| !m.name.starts_with("─"))
        .collect();

    // If components were selected, choose installation target
    let target = if !selected_components.is_empty() {
        let targets = vec![InstallTarget::Host, InstallTarget::DevContainer];
        let selected = Select::new("Where would you like to install these components?", targets)
            .with_help_message("↑↓ to move, enter to select")
            .with_formatter(&|item| {
                format!("{} - {}", item.value, get_target_description(item.value))
            })
            .prompt()?;
        Some(selected)
    } else {
        None
    };

    Ok(InstallPlan {
        target,
        components: selected_components,
    })
}

pub fn confirm_installation(plan: &InstallPlan) -> Result<bool> {
    if plan.components.is_empty() {
        println!("\nNo components selected for installation.");
        return Ok(false);
    }

    println!("\nInstallation Plan:");
    
    // Show installation target if components were selected
    if let Some(target) = &plan.target {
        println!("\n🔹 Installation Target: {}", target);
        println!("  {}", get_target_description(target));
    }

    // Show selected components
    println!("\n🔹 Selected Components:");
    for component in &plan.components {
        println!("  • {}", component.name);
        println!("    Description: {}", component.description);
        println!("    Dependencies: {}", component.dependencies.join(", "));
    }

    let options = vec!["Yes, proceed with installation", "No, let me change my selection"];
    let answer = Select::new("Proceed with installation?", options).prompt()?;

    Ok(answer.starts_with("Yes"))
}
