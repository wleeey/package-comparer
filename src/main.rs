use env_logger::Builder;
use log::LevelFilter;
use package_comparer::branches::Branch;
use package_comparer::{architecture_support, cases};
use std::io::Write;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();

    let architectures = architecture_support::fetch_supported_architectures().await;

    let selected_arch = cli::select_architecture(architectures);

    let only_sisyphus_packages =
        cases::fetch_only_packages_from_selected_branch(Branch::Sisyphus, &selected_arch).await;

    println!("sisyphus");
    for only_sisyphus_package in only_sisyphus_packages {
        println!("{}", only_sisyphus_package.name_ref())
    }

    println!("p10");
    let _only_p10_packages =
        cases::fetch_only_packages_from_selected_branch(Branch::P10, &selected_arch).await;

    for only_p10_package in _only_p10_packages {
        println!("{}", only_p10_package.name_ref())
    }

    Ok(())
}
