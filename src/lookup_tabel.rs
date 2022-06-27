use std::collections::HashMap;

use crate::Error;
use bool_algebra::bool_to_u32;

#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    table: Vec<Vec<bool>>,
    in_values: Vec<bool>,

    in_map: HashMap<String, usize>,
    out_map: HashMap<String, usize>,
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

        let name = name.to_string();

        let mut in_map = HashMap::new();
        let mut out_map = HashMap::new();

        for (i, input) in in_names.iter().enumerate() {
            in_map.insert(input.to_string(), i);
        }

        for (i, output) in out_names.iter().enumerate() {
            out_map.insert(output.to_string(), i);
        }

        Ok(Self {
            table,
            in_values: vec![false; in_names.len()],

            name,
            in_map,
            out_map,
        })
    }

    pub fn get_table(&self) -> Vec<Vec<bool>> {
        self.table.clone()
    }

    pub fn in_map(&self, name: String) -> Option<usize> {
        if let Some(value) = self.in_map.get(&name) {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn out_map(&self, name: String) -> Option<usize> {
        if let Some(value) = self.out_map.get(&name) {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn outputs(&self) -> Vec<bool> {
        let index = bool_to_u32(self.in_values.clone()) as usize;
        let mut result = Vec::with_capacity(self.out_map.len());
        for i in 0..self.out_map.len() {
            result.push(self.table[i][index]);
        }

        return result;
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set(&mut self, in_name: &str, value: bool) -> Result<(), Error> {
        if let Some(index) = self.in_map.get(in_name) {
            self.in_values[index.clone()] = value;
            Ok(())
        } else {
            Err(Error::msg(format!("name {} not found", in_name)))
        }
    }

    pub fn get(&mut self, out_name: &str) -> Result<bool, Error> {
        let index = bool_to_u32(self.in_values.clone()) as usize;
        if let Some(out_id) = self.out_map.get(out_name) {
            Ok(self.table[out_id.clone()][index])
        } else {
            Err(Error::msg(format!("name {} not found", out_name)))
        }
    }

    pub fn set_id(&mut self, in_id: usize, value: bool) -> Result<(), Error> {
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

    pub fn get_id(&mut self, out_id: usize) -> Result<bool, Error> {
        let index = bool_to_u32(self.in_values.clone()) as usize;
        if out_id > self.out_map.len() - 1 {
            Err(Error::msg(format!(
                "out_id dose not exist max {} input {}",
                self.out_map.len() - 1,
                out_id
            )))
        } else {
            Ok(self.table[out_id][index])
        }
    }

    // pub fn in_names(&self) -> Vec<String> {
    //     self.in_map.()
    // }
    //
    // pub fn out_names(&self) -> Vec<String> {
    //     self.out_names.clone()
    // }
}
