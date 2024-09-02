use env_logger::Builder;
use log::LevelFilter;
use package_comparer::branches::Branch;
use package_comparer::{architecture_support, cases};
use std::io::Write;

mod cli;

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();

    let architectures = architecture_support::fetch_supported_architectures().await;

    let selected_arch = cli::select_architecture(architectures);

    let only_sisyphus_packages =
        cases::fetch_only_packages_from_selected_branch(Branch::Sisyphus, &selected_arch).await;

    let only_p10_packages =
        cases::fetch_only_packages_from_selected_branch(Branch::P10, &selected_arch).await;

    let packages_vr_more_in_sisyphus_than_p10 =
        cases::fetch_vr_more_in_sisyphus_than_p10(&selected_arch).await;

    let output = serde_json::json!({
        "only_sisyphus_packages": only_sisyphus_packages,
        "only_p10_packages": only_p10_packages,
        "packages_vr_more_in_sisyphus_than_p10": packages_vr_more_in_sisyphus_than_p10
    });

    println!("{}", output);
}
