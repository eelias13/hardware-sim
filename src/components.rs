use crate::{
    connection::Connection, entry::Entry, in_out::InOut, Circuit, Component, Error, IODevice,
    LookupTable,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Components {
    circuits: Vec<Entry<Circuit>>,
    luts: Vec<Entry<LookupTable>>,
    io: Vec<InOut>,
    in_names: Vec<String>,
    out_names: Vec<String>,
    comp_map: HashMap<String, usize>,
    name: String,
}

impl Components {
    pub fn new(
        in_names: Vec<&str>,
        out_names: Vec<&str>,
        connections: Vec<Connection>,
        lut_map: &HashMap<String, LookupTable>,
        circuit_map: &HashMap<String, Circuit>,
        name: &str,
    ) -> Result<Self, Error> {
        // generate input and output nodesout_name
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
        let in_names: Vec<String> = in_names
            .iter()
            .map(|s| -> String { s.to_string() })
            .collect();
        let out_names: Vec<String> = out_names
            .iter()
            .map(|s| -> String { s.to_string() })
            .collect();
        let name = name.to_string();

        let mut luts = Vec::new();
        let mut circuits = Vec::new();

        let mut lut_comp_name = Vec::new();
        let mut circ_comp_name = Vec::new();

        for connection in connections {
            if let Some(component) = lut_map.get(&connection.name()) {
                if let Some(lut) = component.to_lut() {
                    lut_comp_name.push(component.name());
                    luts.push(Entry::new(lut, connection)?);
                } else {
                    unreachable!();
                }
            } else if let Some(component) = circuit_map.get(&connection.name()) {
                if let Some(circuit) = component.to_circuit() {
                    circuits.push(Entry::new(circuit, connection)?);
                    circ_comp_name.push(component.name());
                } else {
                    unreachable!();
                }
            } else {
                return Err(Error::msg(format!(
                    "no component found named {}",
                    connection.name()
                )));
            }
        }

        for (i, name) in lut_comp_name.iter().enumerate() {
            comp_map.insert(name.to_owned(), io.len() + i);
        }
        for (i, name) in circ_comp_name.iter().enumerate() {
            comp_map.insert(name.to_owned(), io.len() + luts.len() + i);
        }

        Ok(Self {
            circuits,
            in_names,
            io,
            luts,
            out_names,
            comp_map,
            name,
        })
    }

    pub fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error> {
        let mut index = self.index(in_name)?;
        if index <= self.io.len() {
            return self.io[index].set(in_name, value, max_vised);
        }

        index -= self.io.len();
        if index <= self.luts.len() {
            return self.luts[index].set(in_name, value, max_vised);
        }

        index -= self.luts.len();
        if index <= self.circuits.len() {
            return self.circuits[index].set(in_name, value, max_vised);
        }

        unreachable!();
    }

    pub fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error> {
        let mut index = self.index(out_name)?;
        if index <= self.io.len() {
            return self.io[index].get(out_name, max_vised);
        }

        index -= self.io.len();
        if index <= self.luts.len() {
            return self.luts[index].get(out_name, max_vised);
        }

        index -= self.luts.len();
        if index <= self.circuits.len() {
            return self.circuits[index].get(out_name, max_vised);
        }
        unreachable!();
    }

    pub fn get_all_out(&self) -> Vec<bool> {
        let mut result = Vec::new();
        for i in self.in_names.len()..self.io.len() {
            result.push(self.io[i].value());
        }
        result
    }

    pub fn in_names(&self) -> Vec<String> {
        self.in_names.clone()
    }

    pub fn out_names(&self) -> Vec<String> {
        self.out_names.clone()
    }

    pub fn index(&self, name: &str) -> Result<usize, Error> {
        if let Some(&index) = self.comp_map.get(name) {
            Ok(index)
        } else {
            Err(Error::msg(format!(
                "{} not found in {}",
                name,
                self.name.clone()
            )))
        }
    }

    pub fn len(&self) -> usize {
        self.in_names.len() + self.out_names.len() + self.luts.len() + self.circuits.len()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_max_vised(&mut self, max_vised: u8) {
        for circuit in self.circuits.iter_mut() {
            circuit.set_max_vised(max_vised);
        }
    }
}
