use crate::{Component, Error};
use bool_algebra::bool_to_u32;

#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    table: Vec<Vec<bool>>,
    in_values: Vec<bool>,
    in_names: Vec<String>,
    out_names: Vec<String>,
    name: String,
}

impl LookupTable {
    pub fn new(
        table: Vec<Vec<bool>>,
        in_names: Vec<String>,
        out_names: Vec<String>,
        name: String,
    ) -> Result<Self, Error> {
        if table.len() != out_names.len() {
            return Err(Error::msg(format!(
                "wrong shape table.len() <{}> has to equal out_names.len() <{}>",
                table.len(),
                out_names.len()
            )));
        }
        for t in table.iter() {
            if 2_usize.pow(in_names.len() as u32) != t.len() {
                return Err(Error::msg(format!(
                    "wrong shape table[i].len() <{}> has to equal 2^in_names.len() <{}>",
                    2_usize.pow(in_names.len() as u32),
                    t.len()
                )));
            }
        }

        Ok(Self {
            table,
            in_values: vec![false; in_names.len()],
            in_names,
            out_names,
            name,
        })
    }
}

impl Component for LookupTable {
    fn set_max_vised(&mut self, _: u8) {}

    fn to_lut(&self) -> Option<LookupTable> {
        Some(self.clone())
    }

    fn to_circuit(&self) -> Option<crate::Circuit> {
        None
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
        Err(Error::msg(format!(
            "in_names {} not found in chip (lut) {}",
            in_names,
            self.name()
        )))
    }

    fn get(&mut self, out_names: &str) -> Result<bool, Error> {
        let index = bool_to_u32(self.in_values.clone());
        for (i, name) in self.in_names.iter().enumerate() {
            if out_names == name {
                return Ok(self.table[i][index as usize]);
            }
        }
        Err(Error::msg(format!(
            "out_names {} not found in chip (lut) {}",
            out_names,
            self.name()
        )))
    }

    fn in_names(&self) -> Vec<String> {
        self.in_names.clone()
    }
    fn out_names(&self) -> Vec<String> {
        self.out_names.clone()
    }
}
