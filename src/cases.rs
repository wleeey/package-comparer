use crate::api_alt_json_templates::Package;
use crate::architecture_support::Arch;
use crate::branches::Branch;
use rpm::Nevra;
use std::cmp::Ordering;
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

pub async fn fetch_vr_more_in_sisyphus_than_p10(arch: &Arch) -> Vec<Package> {
    let sisyphus_packages =
        crate::fetch_packages_from_branch_for_architecture(Branch::Sisyphus, arch).await;

    let p10_packages = crate::fetch_packages_from_branch_for_architecture(Branch::P10, arch).await;

    let p10_names: HashSet<_> = p10_packages
        .iter()
        .map(|pkg| pkg.name_as_str().to_string())
        .collect();

    sisyphus_packages
        .into_iter()
        .filter(|sisyphus_package| {
            p10_names.contains(&sisyphus_package.name_as_str().to_string())
                && p10_packages.iter().any(|p10_package| {
                    p10_package.name_as_str() == sisyphus_package.name_as_str()
                        && rpm::rpm_evr_compare(
                            &Nevra::new(
                                "",
                                &sisyphus_package.epoch_ref().to_string().as_str(),
                                &sisyphus_package.version_as_str(),
                                &sisyphus_package.release_as_str(),
                                "",
                            )
                            .to_string(),
                            &Nevra::new(
                                "",
                                &p10_package.epoch_ref().to_string().as_str(),
                                &p10_package.version_as_str(),
                                &p10_package.release_as_str(),
                                "",
                            )
                            .to_string(),
                        ) == Ordering::Greater
                })
        })
        .collect::<Vec<Package>>()
}

async fn only_a_packages(packages_a: Vec<Package>, packages_b: Vec<Package>) -> Vec<Package> {
    let packages_b_names = packages_b
        .into_iter()
        .map(|package| package.name_as_str().to_string())
        .collect::<HashSet<String>>();

    packages_a
        .into_iter()
        .filter(|package| !packages_b_names.contains(package.name_as_str()))
        .collect::<Vec<Package>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{architecture_support, Branch};

    #[tokio::test]
    pub async fn only_packages_unique_to_the_branch_are_output() {
        let architectures = architecture_support::fetch_supported_architectures().await;

        let architecture = architectures.iter().next().unwrap();

        let test_packages =
            fetch_only_packages_from_selected_branch(Branch::Sisyphus, architecture).await;

        let packages =
            crate::fetch_packages_from_branch_for_architecture(Branch::P10, architecture).await;

        let package_names = packages
            .into_iter()
            .map(|package| package.name_as_str().to_string())
            .collect::<HashSet<String>>();

        for test_package in test_packages {
            if package_names.contains(&test_package.name_as_str().to_string()) {
                panic!(
                    "Package '{}' exists in both branches!",
                    test_package.name_as_str()
                );
            }
        }
    }
}
