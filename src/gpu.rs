use anyhow::{anyhow, Result};
use std::process::Command;
use which::which;

#[derive(Debug)]
pub enum GpuType {
    Nvidia {
        name: String,
        driver_version: String,
        memory: String,
        cuda_available: bool,
    },
    Metal {
        device_name: String,
        architecture: String,
    },
    None,
}

pub fn detect_gpu() -> Result<GpuType> {
    // First check for NVIDIA GPU
    if which("nvidia-smi").is_ok() {
        let output = Command::new("nvidia-smi")
            .arg("--query-gpu=gpu_name,driver_version,memory.total")
            .arg("--format=csv,noheader")
            .output()?;

        if output.status.success() {
            let info = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = info.split(',').map(|s| s.trim()).collect();
            
            if parts.len() >= 3 {
                return Ok(GpuType::Nvidia {
                    name: parts[0].to_string(),
                    driver_version: parts[1].to_string(),
                    memory: parts[2].to_string(),
                    cuda_available: which("nvcc").is_ok(),
                });
            }
        }
    }

    // Check for Metal (macOS)
    if cfg!(target_os = "macos") {
        // system_profiler requires no special permissions on macOS
        let output = Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .output()?;

        if output.status.success() {
            let info = String::from_utf8_lossy(&output.stdout);
            
            // Parse the Metal GPU info
            if let Some(gpu_line) = info.lines()
                .find(|line| line.contains("Chipset Model:")) {
                let device_name = gpu_line
                    .split(":")
                    .nth(1)
                    .map(|s| s.trim())
                    .unwrap_or("Unknown")
                    .to_string();

                // Get architecture (Apple Silicon vs Intel)
                let arch = if device_name.contains("Apple") {
                    "Apple Silicon"
                } else {
                    "Intel"
                };

                return Ok(GpuType::Metal {
                    device_name,
                    architecture: arch.to_string(),
                });
            }
        }
    }

    Ok(GpuType::None)
}

pub fn detect_and_setup() -> Result<()> {
    println!("🔍 Checking GPU availability");

    match detect_gpu()? {
        GpuType::Nvidia { name, driver_version, memory, cuda_available } => {
            println!("✅ NVIDIA GPU detected:");
            println!("  • Name: {}", name);
            println!("  • Driver: {}", driver_version);
            println!("  • Memory: {}", memory);
            
            if cuda_available {
                println!("✅ CUDA toolkit is installed");
            } else {
                println!("⚠️  NVIDIA GPU found but CUDA toolkit is not installed");
                println!("   Consider installing CUDA toolkit for GPU acceleration");
            }
        },
        GpuType::Metal { device_name, architecture } => {
            println!("✅ Metal GPU detected:");
            println!("  • Device: {}", device_name);
            println!("  • Architecture: {}", architecture);
            println!("  • Metal support: Available");
        },
        GpuType::None => {
            println!("ℹ️  No GPU acceleration detected");
            println!("   Using CPU-only mode");
        }
    }

    Ok(())
}
