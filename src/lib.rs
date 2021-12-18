use bool_algebra::bool_to_u32;
use std::collections::HashMap;

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

trait IODevice {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error>;
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error>;
    fn in_names(&self) -> Vec<String>;
    fn out_names(&self) -> Vec<String>;
}

struct InOut {
    name: String,
    viseted: u8,
    value: bool,
}

impl InOut {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            viseted: 0,
            value: false,
        }
    }
}

impl IODevice for InOut {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error> {
        if in_name != self.name {
            todo!();
        }
        if self.viseted > max_vised {
            todo!();
        }
        self.viseted += 1;
        self.value = value;
        Ok(())
    }
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error> {
        if out_name != self.name {
            todo!();
        }
        if self.viseted > max_vised {
            todo!();
        }
        self.viseted += 1;
        Ok(self.value)
    }
    fn in_names(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
    fn out_names(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
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
}

impl<'a> IODevice for Entry<'a> {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error> {
        if self.viseted > max_vised {
            // TODO
            todo!();
        }
        if let Some(name) = self.in_map.get(in_name) {
            self.viseted += 1;
            self.component.set(name, value)
        } else {
            // TODO
            todo!();
        }
    }
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error> {
        if self.viseted > max_vised {
            // TODO
            todo!();
        }
        if let Some(name) = self.out_map.get(out_name) {
            self.viseted += 1;
            self.component.get(name)
        } else {
            // TODO
            todo!();
        }
    }

    fn in_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for key in self.in_map.keys() {
            result.push(key.clone());
        }
        result
    }
    fn out_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for key in self.out_map.keys() {
            result.push(key.clone());
        }
        result
    }
}

struct Connection {
    in_map: HashMap<String, String>,
    out_map: HashMap<String, String>,
    comp_name: String,
}

impl Connection {
    fn new(comp_name: &str, in_tuple: Vec<(&str, &str)>, out_tuple: Vec<(&str, &str)>) -> Self {
        let mut in_map = HashMap::new();
        for (key, val) in in_tuple {
            in_map.insert(key.to_string(), val.to_string());
        }

        let mut out_map = HashMap::new();
        for (key, val) in out_tuple {
            out_map.insert(key.to_string(), val.to_string());
        }

        Self {
            comp_name: comp_name.to_string(),
            out_map,
            in_map,
        }
    }
}

struct Circuit<'a> {
    in_names: Vec<String>,
    out_names: Vec<String>,
    /// a graph containing the index to the components
    compute_graph: Vec<Vec<usize>>,
    components: Vec<&'a mut dyn IODevice>,
    name: String,
}

impl<'a> Circuit<'a> {
    fn new(
        in_names: Vec<&str>,
        out_names: Vec<&str>,
        name: &str,
        connections: Vec<Connection>,
        all_components: HashMap<String, &'a dyn Component>,
    ) -> Result<Self, Error> {
        // generate input and output nodes
        // comp_map is to look up the index in the components vec by the name

        let mut comp_map = HashMap::new();
        let mut io = Vec::new();
        for in_name in in_names.clone() {
            comp_map.insert(in_name.to_string(), io.len());
            io.push(InOut::new(in_name));
        }
        for out_name in out_names.clone() {
            comp_map.insert(out_name.to_string(), io.len());
            io.push(InOut::new(out_name));
        }

        let mut components: Vec<&'a mut (dyn IODevice + 'a)> = Vec::new();
        for in_out in io {
            components.push(&'a mut in_out.to_owned());
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

        // insert components

        let mut compute_graph = Vec::new();

        Ok(Self {
            in_names,
            out_names,
            compute_graph,
            components,
            name,
        })
    }

    fn opt(&mut self) {
        todo!();
    }
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
        self.out_names.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
