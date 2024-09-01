use crate::api_alt_json_templates::{Package, Packages};
use crate::architecture_support::Arch;
use crate::branches::Branch;
use reqwest::Client;

pub mod api_alt_json_templates;
pub mod architecture_support;
pub mod branches;
pub mod cases;

async fn fetch_packages_from_branch_for_architecture(branch: Branch, arch: &Arch) -> Vec<Package> {
    let url = "https://rdb.altlinux.org/api/export/branch_binary_packages/";

    let client = Client::new();

    if !architecture_support::is_architecture_supported_for_brunch(arch, &branch).await {
        return Vec::new();
    }

    client
        .get(format!("{}{}", url, branch.as_str()))
        .query(&[("arch", arch.inner_ref())])
        .send()
        .await
        .unwrap()
        .json::<Packages>()
        .await
        .unwrap_or_else(|err| panic!("Failed parse Packages: {err}"))
        .packages
}
