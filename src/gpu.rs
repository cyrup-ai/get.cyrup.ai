use anyhow::Result;
use which::which;

pub fn detect_and_setup() -> Result<()> {
    println!("🔍 Checking GPU availability");
    
    if which("nvidia-smi").is_ok() {
        let output = std::process::Command::new("nvidia-smi")
            .arg("--query-gpu=gpu_name,driver_version,memory.total")
            .arg("--format=csv,noheader")
            .output()?;
            
        if output.status.success() {
            println!("✅ NVIDIA GPU detected:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            
            if which("nvcc").is_ok() {
                println!("✅ CUDA toolkit is installed");
            } else {
                println!("⚠️  NVIDIA GPU found but CUDA toolkit is not installed");
            }
        }
    } else {
        println!("ℹ️  No NVIDIA GPU detected");
    }
    
    Ok(())
}
