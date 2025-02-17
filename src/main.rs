use anyhow::Result;
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Components to install (default: all)
    #[arg(short, long)]
    components: Vec<String>,

    /// Skip GPU check
    #[arg(long)]
    skip_gpu: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("Starting Cyrup installer...");

    // TODO: Implement installation logic
    
    Ok(())
}
