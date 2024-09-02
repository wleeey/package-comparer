#[derive(Clone, Copy)]
pub enum Branch {
    Sisyphus,
    P10,
}

impl Branch {
    pub fn as_str(&self) -> &str {
        match self {
            Branch::Sisyphus => "sisyphus",
            Branch::P10 => "p10",
        }
    }
}
