use crate::api_alt_json_templates::Package;
use crate::architecture_support::Arch;
use crate::branches::Branch;
use crate::FetchError;
use std::collections::HashSet;

pub async fn fetch_only_packages_from_a_branch(
    branch_a: Branch,
    branch_b: Branch,
    arch: &Arch,
) -> Result<Vec<Package>, FetchError> {
    let primary_packages =
        crate::fetch_packages_from_branch_for_architecture(branch_a, arch).await?;

    match crate::fetch_packages_from_branch_for_architecture(branch_b, arch).await {
        Ok(packages) => Ok(only_a_packages(primary_packages, packages)),
        Err(err) => Ok(crate::handle_fetch_error(err, primary_packages)),
    }
}

fn only_a_packages(packages_a: Vec<Package>, packages_b: Vec<Package>) -> Vec<Package> {
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
    use std::collections::HashSet;

    #[tokio::test]
    pub async fn only_packages_unique_to_the_branch_are_output() {
        let architectures =
            architecture_support::fetch_supported_architectures(Branch::Sisyphus, Branch::P10)
                .await;

        let architecture = architectures.iter().next().unwrap();

        let test_packages =
            fetch_only_packages_from_a_branch(Branch::Sisyphus, Branch::P10, architecture)
                .await
                .unwrap();

        let packages =
            crate::fetch_packages_from_branch_for_architecture(Branch::P10, architecture)
                .await
                .unwrap();

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
