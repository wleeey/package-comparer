use crate::api_alt_json_templates::{Package, Packages};
use crate::architecture_support::{Arch, API_ALT_URL};
use crate::branches::Branch;
use reqwest::Client;

pub mod api_alt_json_templates;
pub mod architecture_support;
pub mod branches;
pub mod cases;

async fn fetch_packages_from_branch_for_architecture(
    branch: Branch,
    arch: &Arch,
) -> Result<Vec<Package>, FetchError> {
    let client = Client::new();

    if !architecture_support::is_architecture_supported_for_brunch(arch, &branch).await {
        return Err(FetchError::ArchNotSupported {
            branch: branch.as_str().to_string(),
            arch: arch.to_string(),
        });
    }

    Ok(client
        .get(format!("{}{}", API_ALT_URL, branch.as_str()))
        .query(&[("arch", arch.inner_ref())])
        .send()
        .await
        .map_err(|err| -> FetchError { FetchError::Other(err.to_string()) })?
        .json::<Packages>()
        .await
        .map_err(|err| -> FetchError { FetchError::Other(err.to_string()) })?
        .packages)
}

#[derive(Debug)]
pub enum FetchError {
    ArchNotSupported { branch: String, arch: String },
    Other(String),
}
