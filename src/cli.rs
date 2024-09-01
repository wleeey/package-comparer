use inquire::Select;
use package_comparer::Arch;

pub fn select_architecture(architectures: Vec<Arch>) -> SelectedArch {
    let selected_arch = Select::new("Architecture that interests you:", architectures)
        .prompt()
        .expect("It is necessary to choose an architecture");

    SelectedArch(selected_arch)
}

pub struct SelectedArch(Arch);
