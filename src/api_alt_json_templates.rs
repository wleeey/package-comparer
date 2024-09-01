use serde::Deserialize;

#[derive(Deserialize)]
pub struct Package {
    name: String,
    epoch: i32,
    version: String,
    release: String,
    arch: String,
}

impl Package {
    pub fn name_ref(&self) -> &str {
        self.name.as_str()
    }
    pub fn arch_ref(&self) -> &str {
        self.arch.as_str()
    }
}

#[derive(Deserialize)]
pub(crate) struct Packages {
    pub(crate) packages: Vec<Package>,
}
