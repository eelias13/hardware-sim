use crate::{connection::Connection, entry::Entry, in_out::InOut, Component, Error, LookupTable};
use std::collections::HashMap;

// to stupid to use `dyn IODevice` if you add new IODevice type don't forget it hear
#[derive(Debug, Clone, PartialEq)]
pub struct Components {
    // circuits: Vec<Entry<Circuit>>,
    comp_map: HashMap<String, usize>,
    luts: Vec<Entry<LookupTable>>,
    io: Vec<InOut>,
    in_names: Vec<String>,
    out_names: Vec<String>,
}

impl Components {
    pub fn new(
        in_names: Vec<&str>,
        out_names: Vec<&str>,
        connections: Vec<Connection>,
        all_components: &HashMap<String, Box<impl Component + 'static>>,
    ) -> Result<Self, Error> {
        // generate input and output nodes
        // comp_map is to look up the index in the components vec by the name
        let mut io = Vec::new();
        for in_name in in_names.clone() {
            io.push(InOut::new(in_name));
        }
        for out_name in out_names.clone() {
            io.push(InOut::new(out_name));
        }
        let in_names: Vec<String> = in_names
            .iter()
            .map(|s| -> String { s.to_string() })
            .collect();
        let out_names: Vec<String> = out_names
            .iter()
            .map(|s| -> String { s.to_string() })
            .collect();

        let mut luts = Vec::new();
        for connection in connections {
            if let Some(component) = all_components.get(&connection.name()) {
                if let Some(lut) = component.to_lut() {
                    luts.push(Entry::new(Box::new(lut), connection)?);
                //  } else if let Some(circuits) = component.to_circuit() {
                } else {
                    todo!();
                }
            } else {
                todo!();
            }
        }

        let mut comp_map = HashMap::new();

        Ok(Self {
            comp_map,
            in_names,
            io,
            luts,
            out_names,
        })
    }

    pub fn comp(&self, name: &str) {
        let name = name.to_string();
        if self.in_names.contains(&name) {}
    }
}
