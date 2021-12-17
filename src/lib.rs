use bool_algebra::bool_to_u32;
use either::Either;

pub struct Error {
    msg: String,
}

impl Error {
    fn msg(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    table: Vec<bool>,
    in_values: Vec<bool>,
    in_names: Vec<String>,
    out_name: String,
}

impl LookupTable {
    pub fn new(
        table: Vec<bool>,
        in_values: Vec<bool>,
        in_names: Vec<String>,
        out_name: String,
    ) -> Self {
        Self {
            table,
            in_values,
            in_names,
            out_name,
        }
    }

    fn set(&mut self, name: &str, value: bool) -> Result<(), Error> {
        for (i, in_name) in self.in_names.iter().enumerate() {
            if in_name == name {
                self.in_values[i] = value;
                return Ok(());
            }
        }
        todo!();
        return Err(Error::msg(""));
    }

    fn get(&self) -> bool {
        let index = bool_to_u32(self.in_values.clone());
        self.table[index as usize]
    }

    fn output(&self) -> String {
        self.out_name.clone()
    }
}

trait Component  {
    
}

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    viseted: u8,
   // component: dyn Component,
}

struct Circuit {}
