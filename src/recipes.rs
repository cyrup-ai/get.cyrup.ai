use anyhow::{anyhow, Result};
use std::process::Command;

pub struct Recipe {
    name: &'static str,
    packages: Packages,
    required: bool,
}

struct Packages {
    ubuntu: Vec<&'static str>,
    amazon: Vec<&'static str>,
    macos: Vec<&'static str>,
}

const RECIPES: &[Recipe] = &[
    Recipe {
        name: "system-core",
        packages: Packages {
            ubuntu: vec!["build-essential", "curl", "git"],
            amazon: vec!["gcc", "gcc-c++", "make", "curl", "git"],
            macos: vec!["curl", "git"],
        },
        required: true,
    },
    Recipe {
        name: "shell-tools",
        packages: Packages {
            ubuntu: vec!["zsh", "tmux", "htop"],
            amazon: vec!["zsh", "tmux", "htop"],
            macos: vec!["zsh", "tmux", "htop"],
        },
        required: true,
    },
    Recipe {
        name: "dev-tools",
        packages: Packages {
            ubuntu: vec!["ripgrep", "fd-find"],
            amazon: vec!["ripgrep"],
            macos: vec!["ripgrep", "fd"],
        },
        required: true,
    },
];

pub fn install_all(os: &str) -> Result<()> {
    for recipe in RECIPES {
        println!("📦 Installing {}", recipe.name);
        let packages = match os {
            "ubuntu" => &recipe.packages.ubuntu,
            "amazon" => &recipe.packages.amazon,
            "macos" => &recipe.packages.macos,
            _ => return Err(anyhow!("Unsupported OS: {}", os)),
        };

        match os {
            "ubuntu" => {
                Command::new("sudo")
                    .args(["apt-get", "install", "-y"])
                    .args(packages)
                    .status()?;
            }
            "amazon" => {
                Command::new("sudo")
                    .args(["yum", "install", "-y"])
                    .args(packages)
                    .status()?;
            }
            "macos" => {
                Command::new("brew")
                    .args(["install"])
                    .args(packages)
                    .status()?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
