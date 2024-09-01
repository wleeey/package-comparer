use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub async fn fetch_supported_architectures() -> Vec<Arch> {
    let branches = vec!["sisyphus", "p10"];

    let sisyphus_packages = fetch_packages(branches.get(0).unwrap()).await;

    let sisyphus_architectures = extract_architectures_from_packages(&sisyphus_packages);

    let p10_packages = fetch_packages(branches.get(1).unwrap()).await;

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

async fn fetch_packages(branch: &str) -> Vec<Package> {
    let url = "https://rdb.altlinux.org/api/export/branch_binary_packages/";

    let client = Client::new();
    client
        .get(format!("{}{}", url, branch))
        .send()
        .await
        .unwrap()
        .json::<Packages>()
        .await
        .unwrap()
        .packages
}

fn extract_architectures_from_packages(packages: &Vec<Package>) -> Vec<&str> {
    packages
        .iter()
        .map(|package| package.arch.as_str())
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

#[derive(Deserialize)]
struct Package {
    name: String,
    epoch: i32,
    version: String,
    release: String,
    arch: String,
}

#[derive(Deserialize)]
struct Packages {
    packages: Vec<Package>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn architecture_is_supported_by_at_least_one_of_the_branches() {
        let architectures = fetch_supported_architectures().await;

        let branches = vec!["sisyphus", "p10"];

        for architecture in &architectures {
            if !is_architecture_supported_for_brunch(&architecture, branches.get(0).unwrap()).await
                && !is_architecture_supported_for_brunch(&architecture, branches.get(1).unwrap())
                    .await
            {
                panic!(
                    "The architecture is not supported by any of the branches: {}",
                    architecture.inner_ref()
                )
            }
        }
    }

    async fn is_architecture_supported_for_brunch(arch: &Arch, branch: &str) -> bool {
        let client = Client::new();
        let url = "https://rdb.altlinux.org/api/export/branch_binary_packages/";

        let res = client
            .get(format!("{}{}", url, branch))
            .query(&[("arch", arch.inner_ref())])
            .send()
            .await
            .unwrap();

        res.status().is_success()
    }
}
