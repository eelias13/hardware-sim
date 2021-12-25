use crate::{connection::Connection, entry::Entry, in_out::InOut, Component, Error, IODevice};
use std::collections::HashMap;

struct Circuit {
    in_names: Vec<String>,
    out_names: Vec<String>,
    /// a graph containing the index to the components
    compute_graph: Vec<Vec<usize>>,
    components: Vec<Box<dyn IODevice>>,
    name: String,
    max_vised: u8,
    comp_map: HashMap<String, usize>,
}

impl Circuit {
    fn new(
        in_names: Vec<&str>,
        out_names: Vec<&str>,
        name: &str,
        connections: Vec<Connection>,
        all_components: &HashMap<String, Box<impl Component + 'static>>,
    ) -> Result<Self, Error> {
        // generate input and output nodes
        // comp_map is to look up the index in the components vec by the name

        let mut comp_map = HashMap::new();
        let mut components: Vec<Box<dyn IODevice>> = Vec::new();

        for in_name in in_names.clone() {
            comp_map.insert(in_name.to_string(), components.len());
            components.push(Box::new(InOut::new(in_name)));
        }
        for out_name in out_names.clone() {
            comp_map.insert(out_name.to_string(), components.len());
            components.push(Box::new(InOut::new(out_name)));
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
        for connection in connections {
            if let Some(component) = all_components.get(&connection.name()) {
                let in_map = HashMap::new();
                for in_name in component.as_ref().in_names() {}
                let out_map = HashMap::new();
                let entry = Entry::new(component.box_clone(), in_map, out_map)?;
                components.push(Box::new(entry));
            } else {
                todo!();
            }
        }

        let mut compute_graph = Vec::new();

        Ok(Self {
            in_names,
            out_names,
            compute_graph,
            components,
            name,
            max_vised: 10,
            comp_map,
        })
    }

    fn opt(&mut self) {
        todo!();
    }

    fn set_max_vised(&mut self, max_vised: u8) {
        self.max_vised = max_vised;
    }
}

/*
impl<'a> Component for Circuit {
    fn set(&mut self, in_name: &str, value: bool) -> Result<(), Error> {
        for i in 0..self.in_names.len() {
            for name in self.components[i].out_names() {
                if in_name == name {
                    return self.components[i].set(in_name, value, self.max_vised);
                }
            }
        }
        todo!();
    }
    fn get(&mut self, out_name: &str) -> Result<bool, Error> {
        for i in self.in_names.len()..self.out_names.len() {
            for name in self.components[i].out_names() {
                if out_name == name {
                    return self.components[i].get(out_name, self.max_vised);
                }
            }
        }
        todo!();
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
*/
