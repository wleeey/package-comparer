use crate::cli::select_branches;
use compare_packages::api_alt_json_templates::Package;
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

    let (branch_one, branch_two) = select_branches();

    log::info!("Fetching supported architectures...");
    let architectures =
        architecture_support::fetch_supported_architectures(branch_one, branch_two).await;

    let selected_arch = cli::select_architecture(architectures);

    log::info!("Fetching those packages that are only in {branch_one} but not in {branch_two}...");
    let only_sisyphus_packages =
        cases::fetch_only_packages_from_a_branch(branch_one, branch_two, &selected_arch)
            .await
            .unwrap_or_else(fetch_error_for_output);

    log::info!("Fetching those packages that are only in {branch_two} but not in {branch_one}...");
    let only_p10_packages =
        cases::fetch_only_packages_from_a_branch(branch_one, branch_two, &selected_arch)
            .await
            .unwrap_or_else(fetch_error_for_output);

    log::info!("Fetching those packages that have more version-release in {branch_one} than in {branch_two}...");
    let packages_vr_more_in_sisyphus_than_p10 =
        cases::fetch_vr_more_in_a_than_b_branch(branch_one, branch_two, &selected_arch)
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
