mod chip_def;
mod circuit;
mod lookup_tabel;

pub use chip_def::{ChipDef, ComponentDef, ComponentIO, ComponentMap};
pub use circuit::Circuit;
pub use lookup_tabel::LookupTable;

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn msg(msg: String) -> Self {
        Self { msg: msg }
    }
}
