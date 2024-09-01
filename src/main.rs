use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();

    let architectures = package_comparer::fetch_supported_architectures().await;

    let selected_arch = cli::select_architecture(architectures);

    Ok(())
}
