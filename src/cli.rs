use inquire::Select;
use package_comparer::architecture_support::Arch;

pub fn select_architecture(architectures: Vec<Arch>) -> Arch {
    Select::new("Architecture that interests you:", architectures)
        .prompt()
        .expect("It is necessary to choose an architecture")
}
