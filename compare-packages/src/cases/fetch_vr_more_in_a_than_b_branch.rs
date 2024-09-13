use crate::api_alt_json_templates::Package;
use crate::architecture_support::Arch;
use crate::branches::Branch;
use crate::FetchError;
use rpm::Nevra;
use std::cmp::Ordering;
use std::collections::HashSet;

pub async fn fetch_vr_more_in_a_than_b_branch(
    branch_a: Branch,
    branch_b: Branch,
    arch: &Arch,
) -> Result<Vec<Package>, FetchError> {
    let packages_a = crate::fetch_packages_from_branch_for_architecture(branch_a, arch).await?;

    let packages_b = crate::fetch_packages_from_branch_for_architecture(branch_b, arch).await?;

    let b_names = packages_b
        .iter()
        .map(|pkg| pkg.name_ref().to_string())
        .collect::<HashSet<String>>();

    Ok(packages_a
        .into_iter()
        .filter(|sisyphus_package| {
            b_names.contains(sisyphus_package.name_ref())
                && packages_b.iter().any(|p10_package| {
                    p10_package.name_ref() == sisyphus_package.name_ref()
                        && package_version_ordering(sisyphus_package, p10_package)
                            == Ordering::Greater
                })
        })
        .collect::<Vec<Package>>())
}

fn package_version_ordering(package_a: &Package, package_b: &Package) -> Ordering {
    rpm::rpm_evr_compare(
        &Nevra::new(
            "",
            package_a.epoch_ref().to_string().as_str(),
            package_a.version_ref(),
            package_a.release_ref(),
            "",
        )
        .to_string(),
        &Nevra::new(
            "",
            package_b.epoch_ref().to_string().as_str(),
            package_b.version_ref(),
            package_b.release_ref(),
            "",
        )
        .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture_support;

    #[tokio::test]
    async fn packages_with_the_highest_version_release_are_returned() {
        let architectures =
            architecture_support::fetch_supported_architectures(Branch::Sisyphus, Branch::P10)
                .await;

        let architecture = architectures.iter().next().unwrap();

        let test_packages =
            fetch_vr_more_in_a_than_b_branch(Branch::Sisyphus, Branch::P10, architecture)
                .await
                .unwrap();

        let packages =
            crate::fetch_packages_from_branch_for_architecture(Branch::P10, architecture)
                .await
                .unwrap();

        let test_packages_names = test_packages
            .iter()
            .map(|pkg| pkg.name_ref().to_string())
            .collect::<HashSet<String>>();

        for package in packages {
            let p10_name = package.name_ref();

            if test_packages_names.contains(p10_name) {
                let test_package = test_packages
                    .iter()
                    .find(|package| package.name_ref() == p10_name)
                    .unwrap();

                if package_version_ordering(&package, &test_package) != Ordering::Less {
                    panic!("Version of package {} in P10 is not less than the corresponding sisyphus package", p10_name);
                }
            }
        }
    }
}
