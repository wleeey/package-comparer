use compare_packages::architecture_support::Arch;
use inquire::Select;

pub fn select_architecture(architectures: Vec<Arch>) -> Arch {
    Select::new("Architecture that interests you:", architectures)
        .prompt()
        .expect("It is necessary to choose an architecture")
}
