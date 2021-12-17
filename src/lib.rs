use std::collections::HashMap;

use bool_algebra::bool_to_u32;

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

    fn get(&self, out_names: &str) -> Result<bool, Error> {
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

trait Component {
    fn set(&mut self, in_names: &str, value: bool) -> Result<(), Error>;
    fn get(&self, out_names: &str) -> Result<bool, Error>;
    fn in_names(&self) -> Vec<String>;
    fn out_names(&self) -> Vec<String>;
    fn name(&self) -> String;
}

// #[derive(Debug, Clone, PartialEq)]
struct Entry<'a> {
    viseted: u8,
    component: &'a mut dyn Component,
    in_map: HashMap<String, String>,
    out_map: HashMap<String, String>,
}

impl<'a> Entry<'a> {
    fn new(
        component: &'a mut dyn Component,
        in_map: HashMap<String, String>,
        out_map: HashMap<String, String>,
    ) -> Result<Self, Error> {
        // TODO validate
        Ok(Self {
            viseted: 0,
            component,
            in_map,
            out_map,
        })
    }

    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error> {
        // TODO max_vised error
        if let Some(name) = self.in_map.get(in_name) {
            self.viseted += 1;
            self.component.set(name, value)
        }else{
            todo!();
        }
    }
    fn get(&mut self, out_names: &str, max_vised: u8) -> Result<bool, Error> {
        // TODO max_vised error
        self.viseted += 1;
        self.component.get(out_names)
    }

    fn in_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for key in self.in_map.keys() {
            result.push(key.clone());
        }
        result
    }
    fn out_namess(&self) -> Vec<String> {
        let mut result = Vec::new();
        for key in self.out_map.keys() {
            result.push(key.clone());
        }
        result
    }
}

struct Circuit<'a> {
    in_names: Vec<String>,
    out_namess: Vec<String>,
    /// a graph containing the index to the components
    compute_graph: Vec<Vec<usize>>,
    components: Vec<Entry<'a>>,
    name: String,
}

impl<'a> Circuit<'a> {
    fn new() {}
}

impl<'a> Component for Circuit<'a> {
    fn set(&mut self, in_names: &str, value: bool) -> Result<(), Error> {
        Ok(())
    }
    fn get(&self, out_names: &str) -> Result<bool, Error> {
        Ok(false)
    }
    fn in_names(&self) -> Vec<String> {
        self.in_names.clone()
    }
    fn out_names(&self) -> Vec<String> {
        self.out_namess.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
