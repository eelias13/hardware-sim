use crate::{Chip, Component, Error, LookupTable};
use bool_algebra::bool_to_u32;
use either::Either;

#[derive(Debug, Clone, PartialEq)]
struct Component {
    viseted: u8,
    component_def: Component,
    component_impl: Either<ChipImpl, LookupTableImpl>,
}

#[derive(Debug, Clone, PartialEq)]
struct ChipImpl {
    chip_def: Chip,
    in_values: Vec<bool>,
    in_names: Vec<String>,
    out_values: Vec<bool>,
    out_names: Vec<String>,
    compute_graph: Vec<Vec<usize>>,
    components: Vec<ComponentImpl>,
}

impl ChipImpl {
    fn new(chip_def: Chip, components: Vec<ComponentImpl>) {}
}

/// an internal implementation for a lockup table in lib is only the definition
#[derive(Debug, Clone, PartialEq)]
struct LookupTableImpl {
    table: Vec<bool>,
    in_values: Vec<bool>,
    in_names: Vec<String>,
    out_name: String,
}

impl LookupTableImpl {
    fn new(lut_def: LookupTable) -> Self {
        let in_names = lut_def.inputs.to_owned();
        LookupTableImpl {
            table: lut_def.table.to_owned(),
            in_values: vec![false; in_names.len()],
            in_names,
            out_name: lut_def.output.to_owned(),
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

pub struct Circuit {
    chips: Vec<Chip>,
    lut: Vec<LookupTableImpl>,
    compute_graph: Vec<Vec<usize>>,
}

impl Circuit {
    pub fn new(chips: Vec<Chip>, lut: Vec<LookupTable>) -> Result<Self, Error> {
        Ok(Self {
            chips,
            lut: lut
                .iter()
                .map(|lut_def| -> LookupTableImpl { LookupTableImpl::new(lut_def.to_owned()) })
                .collect(),
            compute_graph: Vec::new(),
        })
    }

    pub fn set(&mut self, name: &str, value: bool) -> Result<(), Error> {
        for lt in self.lut.iter_mut() {
            if lt.set(name, value).is_ok() {
                return Ok(());
            }
        }
        todo!();
    }

    pub fn get(&self, name: &str) -> Result<bool, Error> {
        for lt in self.lut.iter() {
            if lt.output() == name {
                return Ok(lt.get());
            }
        }
        todo!();
    }
}

#[test]
fn nand() {
    let mut circuit = Circuit::new(
        Vec::new(),
        vec![LookupTable::new(
            "nand",
            vec!["a", "b"],
            "out",
            vec![true, true, true, false],
        )],
    )
    .unwrap();

    assert_eq!(circuit.get("out"), Ok(true));
    assert_eq!(circuit.set("b", true), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));
    assert_eq!(circuit.set("b", false), Ok(()));
    assert_eq!(circuit.set("a", true), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));
    assert_eq!(circuit.set("b", true), Ok(()));
    assert_eq!(circuit.get("out"), Ok(false));
}

#[test]
fn and_from_nand() {
    let chips = vec![Chip::new(
        "And",
        vec!["a", "b"],
        vec!["out"],
        vec![
            Component::new(vec![("a", "a"), ("b", "b"), ("out", "nand")], "Nand"),
            Component::new(vec![("a", "nand"), ("b", "nand"), ("out", "out")], "Nand"),
        ],
    )];

    let nand = LookupTable::new("nand", vec!["a", "b"], "out", vec![true, true, true, false]);
}


//------------------------------------------------------------------------------------------------------------------------------


use std::{collections::HashMap, fmt::Debug};
pub trait Chip {
    fn eval(&self) -> Vec<bool>;
    fn get_name(&self) -> String;
    fn set_input(&mut self, input: &str, value: bool) -> Result<(), String>;
    fn get_in_names(&self) -> Vec<String>;
    // fn get_output(&self, output: &str) -> Result<bool, String>;
}

#[derive(Debug, Clone)]
pub struct Circuit {
    /*
    comput_graphs: Vec<Vec<String>>,
    components: HashMap<String, Gate>,
    input_names: Vec<String>,
    input_values: Vec<bool>,
    outputs: Vec<String>,
    values: Vec<bool>,
    name: String,*/
    input_names: Vec<String>,
    input_values: Vec<bool>,
    outputs: Vec<String>,
    components: Vec<Entry>,
    name: String,
}

/// this maps the internal name of the Chip io to anoter name inorder to allow reusability of gaites
#[derive(Debug, Clone)]
pub struct Entry {
    inputs: Vec<String>,
    outputs: Vec<String>,
    chip: Gate,
}

impl Entry {
    pub fn new(inputs: Vec<&str>, outputs: Vec<&str>, chip: Gate) -> Self {
        Self {
            inputs: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            outputs: outputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            chip: chip,
        }
    }
}

impl Circuit {
    pub fn eval(&self) -> Vec<bool> {
        Vec::new()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_input(&mut self, input: &str, value: bool) -> Result<(), String> {
        let mut found = false;
        for i in 0..self.input_names.len() {
            if input == self.input_names[i] {
                found = true;
                self.input_values[i] = value;
            }
        }

        if !found {
            return Err(format!("input {} not found in gate {}", input, self.name));
        }
        Ok(())
    }
}

impl Circuit {
    pub fn new(
        inputs: Vec<&str>,
        outputs: Vec<&str>,
        components: Vec<Entry>,
        name: &str,
    ) -> Result<Self, String> {
        Ok(Self {
            input_names: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            input_values: vec![false; inputs.len()],
            outputs: outputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            components,
            name: name.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Gate {
    input_names: Vec<String>,
    input_values: Vec<bool>,
    truth_table: Vec<bool>,
    output: String,
    name: String,
}

impl Gate {
    pub fn eval(&self) -> bool {
        let index = bool_algebra::bool_to_u32(self.input_values.iter().cloned().rev().collect());
        self.truth_table[index as usize]
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    pub fn set_input(&mut self, input: &str, value: bool) -> Result<(), String> {
        let mut found = false;
        for i in 0..self.input_names.len() {
            if input == self.input_names[i] {
                found = true;
                self.input_values[i] = value;
            }
        }

        if !found {
            return Err(format!("input {} not found in gate {}", input, self.name));
        }
        Ok(())
    }
}

impl Gate {
    pub fn new(
        inputs: Vec<&str>,
        output: &str,
        truth_table: Vec<bool>,
        name: &str,
    ) -> Result<Self, String> {
        if 2_usize.pow(inputs.len() as u32) != truth_table.len() {
            return Err(format!(
                "incorect size of truth_table expected {} but got {}",
                2_usize.pow(inputs.len() as u32),
                truth_table.len()
            ));
        }

        Ok(Self {
            truth_table,
            output: output.to_string(),
            input_names: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            input_values: vec![false; inputs.len()],
            name: name.to_string(),
        })
    }
}
