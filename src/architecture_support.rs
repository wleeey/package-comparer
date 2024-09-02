use crate::api_alt_json_templates::{Package, Packages};
use crate::branches::Branch;
use reqwest::Client;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub(crate) const API_ALT_URL: &str = "https://rdb.altlinux.org/api/export/branch_binary_packages/";

pub async fn fetch_supported_architectures() -> Vec<Arch> {
    let sisyphus_packages = fetch_packages(Branch::Sisyphus).await;

    let sisyphus_architectures = extract_architectures_from_packages(&sisyphus_packages);

    let p10_packages = fetch_packages(Branch::P10).await;

    let p10_arches = extract_architectures_from_packages(&p10_packages);

    let architectures = sisyphus_architectures
        .iter()
        .chain(p10_arches.iter())
        .collect::<HashSet<_>>();

    architectures
        .into_iter()
        .map(|name| Arch(name.to_string()))
        .collect()
}

pub(crate) async fn is_architecture_supported_for_brunch(arch: &Arch, branch: &Branch) -> bool {
    let client = Client::new();
    let res = client
        .get(format!("{}{}", API_ALT_URL, branch.as_str()))
        .query(&[("arch", arch.inner_ref())])
        .send()
        .await
        .unwrap();

    res.status().is_success()
}

async fn fetch_packages(branch: Branch) -> Vec<Package> {
    reqwest::get(format!("{}{}", API_ALT_URL, branch.as_str()))
        .await
        .unwrap()
        .json::<Packages>()
        .await
        .unwrap()
        .packages
}

fn extract_architectures_from_packages(packages: &[Package]) -> Vec<&str> {
    packages
        .iter()
        .map(|package| package.arch_ref())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub struct Arch(String);

impl Arch {
    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.inner_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn architecture_is_supported_by_at_least_one_of_the_branches() {
        let architectures = fetch_supported_architectures().await;

        for architecture in &architectures {
            if !is_architecture_supported_for_brunch(&architecture, &Branch::Sisyphus).await
                && !is_architecture_supported_for_brunch(&architecture, &Branch::P10).await
            {
                panic!(
                    "The architecture is not supported by any of the branches: {}",
                    architecture.inner_ref()
                )
            }
        }
    }
}
