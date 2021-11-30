use std::collections::HashMap;
pub trait Chip {
    fn eval(&self) -> Vec<bool>;
    fn get_name(&self) -> String;
    fn set_input(&mut self, input: &str, value: bool) -> Result<(), String>;
    // fn get_output(&self, output: &str) -> Result<bool, String>;
}

pub struct Circuit {
    comput_graphs: Vec<Vec<String>>,
    components: HashMap<String, Box<dyn Chip>>,
    inputs: HashMap<String, bool>,
    outputs: Vec<String>,
    values: Vec<bool>,
    name: String,
}

pub struct Gate {
    inputs: Vec<(String, bool)>,
    truth_table: Vec<bool>,
    name: String,
}

impl Gate {
    pub fn eval(&self) -> bool {
        let index = bool_algebra::bool_to_u32(
            self.inputs
                .iter()
                .cloned()
                .rev()
                .map(|(_, b)| -> bool { b })
                .collect(),
        );
        self.truth_table[index as usize]
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_input(&mut self, input: &str, value: bool) -> Result<(), String> {
        let mut found = false;
        for i in 0..self.inputs.len() {
            if input == self.inputs[i].0 {
                found = true;
                self.inputs[i].1 = value;
            }
        }
        if !found {
            return Err(format!("input {} not found in gate {}", input, self.name));
        }
        Ok(())
    }
}

impl Gate {
    pub fn new(inputs: Vec<&str>, truth_table: Vec<bool>, name: &str) -> Result<Self, String> {
        if 2_usize.pow(inputs.len() as u32) != truth_table.len() {
            return Err(format!(
                "incorect size of truth_table expected {} but got {}",
                2_usize.pow(inputs.len() as u32),
                truth_table.len()
            ));
        }

        Ok(Self {
            truth_table,
            inputs: inputs
                .iter()
                .cloned()
                .map(|s| -> (String, bool) { (s.to_string(), false) })
                .collect(),
            name: name.to_string(),
        })
    }
}
