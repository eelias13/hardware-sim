use crate::{components::Components, Component, Connection, Error};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    /// a graph containing the index to the components
    compute_graph: Vec<Vec<usize>>,
    components: Components,
    name: String,
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
        let name = name.to_string();
        let components = Components::new(in_names, out_names, connections, all_components)?;

        let mut compute_graph = Vec::new();

        Ok(Self {
            components,
            compute_graph,
            name,
            max_vised: 10,
        })
    }

    pub fn opt(&mut self) {
        todo!();
    }

    pub fn set_max_vised(&mut self, max_vised: u8) {
        self.max_vised = max_vised;
    }

    pub fn max_vised(&self) -> u8 {
        self.max_vised
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
