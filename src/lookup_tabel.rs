use crate::{Component, Error};
use bool_algebra::bool_to_u32;

#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    table: Vec<bool>,
    in_values: Vec<bool>,
    in_names: Vec<String>,
    out_names: String,
    name: String,
}

impl LookupTable {
    pub fn new(
        table: Vec<bool>,
        in_values: Vec<bool>,
        in_names: Vec<String>,
        out_names: String,
        name: String,
    ) -> Self {
        Self {
            table,
            in_values,
            in_names,
            out_names,
            name,
        }
    }

    fn output(&self) -> String {
        self.out_names.clone()
    }
}

impl Component for LookupTable {
    fn box_clone(&self) -> Box<Self> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn set(&mut self, in_names: &str, value: bool) -> Result<(), Error> {
        for (i, name) in self.in_names.iter().enumerate() {
            if in_names == name {
                self.in_values[i] = value;
                return Ok(());
            }
        }
        // TODO usefull error
        return Err(Error::msg(""));
    }

    fn get(&mut self, out_names: &str) -> Result<bool, Error> {
        let index = bool_to_u32(self.in_values.clone());
        // TODO check name and index
        Ok(self.table[index as usize])
    }

    fn in_names(&self) -> Vec<String> {
        self.in_names.clone()
    }
    fn out_names(&self) -> Vec<String> {
        vec![self.out_names.clone()]
    }
}
