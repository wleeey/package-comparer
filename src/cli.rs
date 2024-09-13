use compare_packages::architecture_support::Arch;
use compare_packages::branches::Branch;
use inquire::Select;

pub fn select_architecture(architectures: Vec<Arch>) -> Arch {
    Select::new("Architecture that interests you:", architectures)
        .prompt()
        .expect("It is necessary to choose an architecture")
}

pub fn select_branches() -> (Branch, Branch) {
    let mut branches: Vec<Branch> = vec![Branch::Sisyphus, Branch::P9, Branch::P10, Branch::P11];

    let branch_one = Select::new("Branch you want to compare:", branches.clone())
        .prompt()
        .expect("It is necessary to choose an branch");

    branches.retain(|branch| branch.as_str() != branch_one.as_str());

    let branch_two = Select::new("Branch you want to compare it with:", branches)
        .prompt()
        .expect("It is necessary to choose an branch");

    (branch_one, branch_two)
}
