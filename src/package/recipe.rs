use anyhow::{Context, Result};
use std::process::Command;
use std::time::Duration;

use super::{manager::PackageManager, Package};

const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY_MS: u64 = 1000;

/// Recipe for installing packages with a specific package manager
#[derive(Debug)]
pub struct Recipe {
    /// Steps to update the package manager
    pub(crate) update_steps: Vec<(&'static str, Vec<&'static str>)>,
    /// Command and args for installing packages
    pub(crate) install_command: (&'static str, Vec<&'static str>),
}

impl Recipe {
    /// Create a new empty recipe
    pub fn new() -> Self {
        Self {
            update_steps: Vec::new(),
            install_command: ("", vec![]),
        }
    }

    /// Set the update steps for this recipe
    pub fn with_update_steps(mut self, steps: Vec<(&'static str, Vec<&'static str>)>) -> Self {
        self.update_steps = steps;
        self
    }

    /// Set the install command for this recipe
    pub fn with_install_step(mut self, cmd: &'static str, args: Vec<&'static str>) -> Self {
        self.install_command = (cmd, args);
        self
    }

    /// Execute this recipe to install packages
    pub fn execute(&self, pm: &PackageManager, packages: &[Package]) -> Result<()> {
        // First run update steps
        for (cmd, args) in &self.update_steps {
            self.retry_with_backoff(|| self.run_cmd(cmd, args), MAX_RETRIES)
                .with_context(|| format!("Failed to update package manager: {} {:?}", cmd, args))?;
        }

        // Then install packages
        let (cmd, base_args) = &self.install_command;
        let mut install_args = base_args.clone();

        // Add package names, using the appropriate name for this package manager
        for package in packages {
            install_args.push(package.name_for(pm));
        }

        self.retry_with_backoff(|| self.run_cmd(cmd, &install_args), MAX_RETRIES)
            .with_context(|| format!("Failed to install packages: {:?}", packages))?;

        Ok(())
    }

    /// Run a system command
    fn run_cmd(&self, cmd: &str, args: &[&str]) -> Result<()> {
        println!("Running: {} {:?}", cmd, args);

        let output = Command::new(cmd)
            .args(args)
            .output()
            .with_context(|| format!("Failed to execute command: {} {:?}", cmd, args))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(anyhow::anyhow!(
                "Command failed with status {}: {} {:?}\nstdout: {}\nstderr: {}",
                output.status,
                cmd,
                args,
                stdout,
                stderr
            ));
        }

        Ok(())
    }

    /// Retry a command with exponential backoff
    fn retry_with_backoff<F>(&self, mut f: F, max_retries: u32) -> Result<()>
    where
        F: FnMut() -> Result<()>,
    {
        let mut retries = 0;
        let mut delay = INITIAL_RETRY_DELAY_MS;

        loop {
            match f() {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if retries >= max_retries {
                        return Err(e.context("Exceeded maximum retry attempts"));
                    }
                    eprintln!("Command failed, retrying in {}ms: {}", delay, e);
                    std::thread::sleep(Duration::from_millis(delay));
                    retries += 1;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_builder() {
        let recipe = Recipe::new()
            .with_update_steps(vec![
                ("apt-get", vec!["update"]),
                ("apt-get", vec!["upgrade"]),
            ])
            .with_install_step("apt-get", vec!["install", "-y"]);

        assert_eq!(recipe.update_steps.len(), 2);
        assert_eq!(recipe.install_command.0, "apt-get");
        assert_eq!(recipe.install_command.1, vec!["install", "-y"]);
    }
}
