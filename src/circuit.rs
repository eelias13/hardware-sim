use crate::{components::Components, Component, Connection, Error};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    /// a graph containing the index to the components
    compute_graph: Vec<Vec<usize>>,
    components: Components,
    max_vised: u8,
}

impl Circuit {
    pub fn new(
        in_names: Vec<&str>,
        out_names: Vec<&str>,
        name: &str,
        connections: Vec<Connection>,
        all_components: &HashMap<String, Box<impl Component + 'static>>,
    ) -> Result<Self, Error> {
        let components = Components::new(
            in_names,
            out_names,
            connections.clone(),
            all_components,
            name,
        )?;

        let mut compute_graph = vec![Vec::new(); components.len()];
        for connection in connections {
            let mut out_index = Vec::new();
            for name in connection.out_map().values() {
                out_index.push(components.index(name)?);
            }

            for name in connection.in_map().values() {
                compute_graph[components.index(name)?] = out_index.clone();
            }
        }

        Ok(Self {
            components,
            compute_graph,
            max_vised: 10,
        })
    }

    pub fn opt(&mut self) {
        todo!();
    }

    pub fn all_max_vised(&mut self, max_vised: u8) {
        self.max_vised = max_vised;
        self.components.set_max_vised(max_vised);
    }

    pub fn max_vised(&self) -> u8 {
        self.max_vised
    }
}

impl Component for Circuit {
    fn set_max_vised(&mut self, max_vised: u8) {
        self.max_vised = max_vised;
    }

    fn set(&mut self, in_name: &str, value: bool) -> Result<(), Error> {
        self.components.set(in_name, value, self.max_vised)
    }

    fn get(&mut self, out_name: &str) -> Result<bool, Error> {
        if self.components.out_names().contains(&out_name.to_string()) {
            self.components.get(out_name, self.max_vised)
        } else {
            Err(Error::msg(format!("{} is not a output", out_name)))
        }
    }
    fn in_names(&self) -> Vec<String> {
        self.components.in_names()
    }

    fn out_names(&self) -> Vec<String> {
        self.components.out_names()
    }
    fn name(&self) -> String {
        self.components.name()
    }

    fn to_lut(&self) -> Option<crate::LookupTable> {
        None
    }

    fn to_circuit(&self) -> Option<Circuit> {
        Some(self.clone())
    }
}
