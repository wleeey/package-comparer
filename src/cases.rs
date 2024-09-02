use crate::api_alt_json_templates::Package;
use crate::architecture_support::Arch;
use crate::branches::Branch;
use crate::FetchError;
use rpm::Nevra;
use std::cmp::Ordering;
use std::collections::HashSet;

pub async fn fetch_only_packages_from_selected_branch(
    branch: Branch,
    arch: &Arch,
) -> Result<Vec<Package>, FetchError> {
    let primary_packages = crate::fetch_packages_from_branch_for_architecture(branch, arch).await?;

    match crate::fetch_packages_from_branch_for_architecture(get_alternate_branch(branch), arch)
        .await
    {
        Ok(packages) => Ok(only_a_packages(primary_packages, packages)),
        Err(err) => Ok(handle_fetch_error(err, primary_packages)),
    }
}

pub fn handle_fetch_error(fetch_error: FetchError, packages: Vec<Package>) -> Vec<Package> {
    match fetch_error {
        FetchError::ArchNotSupported { branch, arch } => {
            log::info!("Architectures {arch} is not supported for {branch} branch");
            packages
        }
        FetchError::Other(message) => {
            panic!("{}", message)
        }
    }
}

fn get_alternate_branch(branch: Branch) -> Branch {
    match branch {
        Branch::Sisyphus => Branch::P10,
        Branch::P10 => Branch::Sisyphus,
    }
}
pub async fn fetch_vr_more_in_sisyphus_than_p10(arch: &Arch) -> Result<Vec<Package>, FetchError> {
    let sisyphus_packages =
        crate::fetch_packages_from_branch_for_architecture(Branch::Sisyphus, arch).await?;

    let p10_packages =
        crate::fetch_packages_from_branch_for_architecture(Branch::P10, arch).await?;

    let p10_names: HashSet<_> = p10_packages
        .iter()
        .map(|pkg| pkg.name_ref().to_string())
        .collect();

    Ok(sisyphus_packages
        .into_iter()
        .filter(|sisyphus_package| {
            p10_names.contains(sisyphus_package.name_ref())
                && p10_packages.iter().any(|p10_package| {
                    p10_package.name_ref() == sisyphus_package.name_ref()
                        && rpm::rpm_evr_compare(
                            &Nevra::new(
                                "",
                                sisyphus_package.epoch_ref().to_string().as_str(),
                                sisyphus_package.version_ref(),
                                sisyphus_package.release_ref(),
                                "",
                            )
                            .to_string(),
                            &Nevra::new(
                                "",
                                p10_package.epoch_ref().to_string().as_str(),
                                p10_package.version_ref(),
                                p10_package.release_ref(),
                                "",
                            )
                            .to_string(),
                        ) == Ordering::Greater
                })
        })
        .collect::<Vec<Package>>())
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

    #[tokio::test]
    pub async fn only_packages_unique_to_the_branch_are_output() {
        let architectures = architecture_support::fetch_supported_architectures().await;

        let architecture = architectures.iter().next().unwrap();

        let test_packages =
            fetch_only_packages_from_selected_branch(Branch::Sisyphus, architecture)
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
