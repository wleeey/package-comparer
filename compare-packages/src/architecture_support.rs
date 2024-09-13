use crate::api_alt_json_templates::Packages;
use crate::branches::Branch;
use reqwest::Client;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub(crate) const API_ALT_URL: &str = "https://rdb.altlinux.org/api/export/branch_binary_packages/";

pub async fn fetch_supported_architectures(branch_a: Branch, branch_b: Branch) -> Vec<Arch> {
    let architectures_branch_a = fetch_architectures_from_branch(&branch_a).await;
    let architectures_branch_b = fetch_architectures_from_branch(&branch_b).await;

    let architectures = architectures_branch_a
        .into_iter()
        .chain(architectures_branch_b)
        .collect::<HashSet<String>>();

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

async fn fetch_architectures_from_branch(branch: &Branch) -> HashSet<String> {
    let packages = reqwest::get(format!("{}{}", API_ALT_URL, branch.as_str()))
        .await
        .unwrap()
        .json::<Packages>()
        .await
        .unwrap()
        .packages;

    let architectures = packages
        .iter()
        .map(|package| package.arch_ref().to_string())
        .collect::<HashSet<String>>();

    architectures
}

pub struct Arch(String);

impl Arch {
    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn architecture_is_supported_by_at_least_one_of_the_branches() {
        let architectures = fetch_supported_architectures(Branch::Sisyphus, Branch::P10).await;

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
