use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum Branch {
    Sisyphus,
    P9,
    P10,
    P11,
}

impl Branch {
    pub fn as_str(&self) -> &str {
        match self {
            Branch::Sisyphus => "sisyphus",
            Branch::P9 => "p9",
            Branch::P10 => "p10",
            Branch::P11 => "p11",
        }
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
