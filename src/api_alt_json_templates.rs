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
    pub fn name_as_str(&self) -> &str {
        self.name.as_str()
    }
    pub fn arch_as_str(&self) -> &str {
        self.arch.as_str()
    }
    pub fn epoch_ref(&self) -> &i32 {
        &self.epoch
    }
    pub fn version_as_str(&self) -> &str {
        self.version.as_str()
    }
    pub fn release_as_str(&self) -> &str {
        self.release.as_str()
    }
}

#[derive(Deserialize)]
pub(crate) struct Packages {
    pub(crate) packages: Vec<Package>,
}
