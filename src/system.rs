use anyhow::{anyhow, Result};
use std::process::Command;
use which::which;

pub fn ensure_sudo_access() -> Result<()> {
    if whoami::username() == "root" {
        return Ok(());
    }

    if which("sudo").is_err() {
        return Err(anyhow!("sudo is not installed. Please run as root or install sudo."));
    }

    match run_cmd("sudo", &["-n", "true"]) {
        Ok(_) => println!("Sudo access verified"),
        Err(_) => {
            println!("Please enter your sudo password (will be cached)");
            run_cmd("sudo", &["-v"])?;
            run_cmd("bash", &["-c", "while true; do sudo -n true; sleep 50; kill -0 $$ || exit; done 2>/dev/null &"])?;
        }
    }
    Ok(())
}

pub fn detect_platform() -> Result<(String, String)> {
    let arch = std::env::consts::ARCH.to_string();
    
    match std::env::consts::OS {
        "macos" => Ok(("macos".to_string(), arch)),
        "linux" => {
            let distro = if std::fs::read_to_string("/etc/os-release")?
                .lines()
                .any(|line| line.contains("ID=") && (line.contains("amzn") || line.contains("amazon")))
            {
                "amazon"
            } else {
                "ubuntu"
            };
            Ok((distro.to_string(), arch))
        }
        os => Err(anyhow!("Unsupported OS: {}", os))
    }
}

pub fn setup(os: &str) -> Result<()> {
    match os {
        "ubuntu" => {
            run_cmd("sudo", &["apt-get", "update"])?;
            run_cmd("sudo", &["apt-get", "upgrade", "-y"])?;
        }
        "amazon" => {
            run_cmd("sudo", &["yum", "update", "-y"])?;
        }
        "macos" => {
            if which("brew").is_err() {
                run_cmd("bash", &["-c", "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""])?;
            }
            run_cmd("brew", &["update"])?;
        }
        _ => return Err(anyhow!("Unsupported OS: {}", os))
    }
    Ok(())
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|_| ())
        .map_err(|e| anyhow!("Failed to run {}: {}", cmd, e))
}
