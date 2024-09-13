use compare_packages::api_alt_json_templates::Package;
use compare_packages::{architecture_support, cases, FetchError};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::{fs, io};

mod cli;

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();

    let (branch_one, branch_two) = cli::select_branches();

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
        format!("only_{branch_one}_packages"): only_sisyphus_packages,
        format!("only_{branch_two}_packages"): only_p10_packages,
        format!("packages_vr_more_in_{branch_one}_than_{branch_two}"): packages_vr_more_in_sisyphus_than_p10
    });

    let mut file_name = String::new();
    println!("Enter the name of the file in which the program result will be written:");
    io::stdin()
        .read_line(&mut file_name)
        .expect("File name entered by the user could not be read");

    let file_name = file_name.trim();

    fs::write(
        format!("{file_name}.json"),
        serde_json::to_string_pretty(&output).unwrap(),
    )
    .expect("Error writing file to output file");

    log::info!("Result of the program was recorded in {file_name}.json")
}

fn fetch_error_for_output(fetch_fn: FetchError) -> Vec<Package> {
    compare_packages::handle_fetch_error(fetch_fn, vec![])
}
