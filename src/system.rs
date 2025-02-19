use anyhow::{anyhow, Result};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Command failed: {} {:?}", cmd, args));
    }

    Ok(())
}

pub fn retry_with_backoff<F>(mut operation: F, max_attempts: u32) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    let mut attempts = 0;
    loop {
        match operation() {
            Ok(_) => return Ok(()),
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(e);
                }
                let backoff = Duration::from_secs(2u64.pow(attempts - 1));
                thread::sleep(backoff);
            }
        }
    }
}
