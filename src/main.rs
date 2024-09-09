use compare_packages::api_alt_json_templates::Package;
use compare_packages::branches::Branch;
use compare_packages::{architecture_support, cases, FetchError};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

mod cli;

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();

    log::info!("Fetching supported architectures...");
    let architectures = architecture_support::fetch_supported_architectures().await;

    let selected_arch = cli::select_architecture(architectures);

    log::info!("Fetching those packages that are only in sisyphus but not in p10...");
    let only_sisyphus_packages =
        cases::fetch_only_packages_from_selected_branch(Branch::Sisyphus, &selected_arch)
            .await
            .unwrap_or_else(fetch_error_for_output);

    log::info!("Fetching those packages that are only in p10 but not in sisyphus...");
    let only_p10_packages =
        cases::fetch_only_packages_from_selected_branch(Branch::P10, &selected_arch)
            .await
            .unwrap_or_else(fetch_error_for_output);

    log::info!("Fetching those packages that have more version-release in sisyphus than in p10...");
    let packages_vr_more_in_sisyphus_than_p10 =
        cases::fetch_vr_more_in_sisyphus_than_p10(&selected_arch)
            .await
            .unwrap_or_else(fetch_error_for_output);

    let output = serde_json::json!({
        "only_sisyphus_packages": only_sisyphus_packages,
        "only_p10_packages": only_p10_packages,
        "packages_vr_more_in_sisyphus_than_p10": packages_vr_more_in_sisyphus_than_p10
    });

    println!("{}", output);
}

fn fetch_error_for_output(fetch_fn: FetchError) -> Vec<Package> {
    compare_packages::handle_fetch_error(fetch_fn, vec![])
}
