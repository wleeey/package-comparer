use crate::api_alt_json_templates::Package;
use crate::architecture_support::Arch;
use crate::branches::Branch;
use std::collections::HashSet;

pub async fn fetch_only_packages_from_selected_branch(branch: Branch, arch: &Arch) -> Vec<Package> {
    let sisyphus_packages =
        crate::fetch_packages_from_branch_for_architecture(Branch::Sisyphus, arch).await;

    let p10_packages = crate::fetch_packages_from_branch_for_architecture(Branch::P10, arch).await;

    match branch {
        Branch::Sisyphus => only_a_packages(sisyphus_packages, p10_packages).await,
        Branch::P10 => only_a_packages(p10_packages, sisyphus_packages).await,
    }
}

async fn only_a_packages(packages_a: Vec<Package>, packages_b: Vec<Package>) -> Vec<Package> {
    let packages_b_names = packages_b
        .into_iter()
        .map(|package| package.name_ref().to_string())
        .collect::<HashSet<String>>();

    packages_a
        .into_iter()
        .filter(|package| !packages_b_names.contains(package.name_ref()))
        .collect::<Vec<Package>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{architecture_support, Branch};

    #[tokio::test]
    pub async fn only_packages_unique_to_the_branch_are_output() {
        let architectures = architecture_support::fetch_supported_architectures().await;

        let test_packages = fetch_only_packages_from_selected_branch(
            Branch::Sisyphus,
            architectures.iter().next().unwrap(),
        )
        .await;

        let packages = crate::fetch_packages_from_branch_for_architecture(
            Branch::P10,
            architectures.iter().next().unwrap(),
        )
        .await;

        let package_names = packages
            .into_iter()
            .map(|package| package.name_ref().to_string())
            .collect::<HashSet<String>>();

        for test_package in test_packages {
            if package_names.contains(&test_package.name_ref().to_string()) {
                panic!(
                    "Package '{}' exists in both branches!",
                    test_package.name_ref()
                );
            }
        }
    }
}
