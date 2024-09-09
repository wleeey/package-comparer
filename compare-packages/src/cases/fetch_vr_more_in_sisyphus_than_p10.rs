use crate::api_alt_json_templates::Package;
use crate::architecture_support::Arch;
use crate::branches::Branch;
use crate::FetchError;
use rpm::Nevra;
use std::cmp::Ordering;
use std::collections::HashSet;

pub async fn fetch_vr_more_in_sisyphus_than_p10(arch: &Arch) -> Result<Vec<Package>, FetchError> {
    let sisyphus_packages =
        crate::fetch_packages_from_branch_for_architecture(Branch::Sisyphus, arch).await?;

    let p10_packages =
        crate::fetch_packages_from_branch_for_architecture(Branch::P10, arch).await?;

    let p10_names = p10_packages
        .iter()
        .map(|pkg| pkg.name_ref().to_string())
        .collect::<HashSet<String>>();

    Ok(sisyphus_packages
        .into_iter()
        .filter(|sisyphus_package| {
            p10_names.contains(sisyphus_package.name_ref())
                && p10_packages.iter().any(|p10_package| {
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
        let architectures = architecture_support::fetch_supported_architectures().await;

        let architecture = architectures.iter().next().unwrap();

        let test_packages = fetch_vr_more_in_sisyphus_than_p10(architecture)
            .await
            .unwrap();

        let p10_packages =
            crate::fetch_packages_from_branch_for_architecture(Branch::P10, architecture)
                .await
                .unwrap();

        let test_packages_names = test_packages
            .iter()
            .map(|pkg| pkg.name_ref().to_string())
            .collect::<HashSet<String>>();

        for p10_package in p10_packages {
            let p10_name = p10_package.name_ref();

            if test_packages_names.contains(p10_name) {
                let test_package = test_packages
                    .iter()
                    .find(|package| package.name_ref() == p10_name)
                    .unwrap();

                if package_version_ordering(&p10_package, &test_package) != Ordering::Less {
                    panic!("Version of package {} in P10 is not less than the corresponding sisyphus package", p10_name);
                }
            }
        }
    }
}
