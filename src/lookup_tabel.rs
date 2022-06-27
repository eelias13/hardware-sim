// use crate::{Component, Error};
use crate::Error;
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
        in_names: Vec<&str>,
        out_names: Vec<&str>,
        name: &str,
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

        let in_names: Vec<String> = in_names
            .iter()
            .map(|s| -> String { s.to_string() })
            .collect();
        let out_names: Vec<String> = out_names
            .iter()
            .map(|s| -> String { s.to_string() })
            .collect();
        let name = name.to_string();

        Ok(Self {
            table,
            in_values: vec![false; in_names.len()],
            in_names,
            out_names,
            name,
        })
    }

    pub fn get_table(&self) -> Vec<Vec<bool>> {
        self.table.clone()
    }

    pub fn outputs(&self) -> Vec<bool> {
        let index = bool_to_u32(self.in_values.clone()) as usize;
        let mut result = Vec::with_capacity(self.out_names.len());
        for i in 0..self.out_names.len() {
            result.push(self.table[i][index]);
        }

        return result;
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set(&mut self, in_id: usize, value: bool) -> Result<(), Error> {
        if in_id > self.in_values.len() - 1 {
            Err(Error::msg(format!(
                "in_id dose not exist max {} input {}",
                self.in_values.len() - 1,
                in_id
            )))
        } else {
            self.in_values[in_id] = value;
            Ok(())
        }
    }

    pub fn get(&mut self, out_id: usize) -> Result<bool, Error> {
        let index = bool_to_u32(self.in_values.clone()) as usize;
        if out_id > self.out_names.len() - 1 {
            Err(Error::msg(format!(
                "out_id dose not exist max {} input {}",
                self.out_names.len() - 1,
                out_id
            )))
        } else {
            Ok(self.table[out_id][index])
        }
    }

    pub fn in_names(&self) -> Vec<String> {
        self.in_names.clone()
    }

    pub fn out_names(&self) -> Vec<String> {
        self.out_names.clone()
    }
}
