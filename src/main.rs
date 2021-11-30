use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut and = Gate::new(
        vec![("a", false), ("b", false)],
        vec![false, false, false, true],
        "and",
    )
    .unwrap();
}

trait Chip {
    fn eval(&self) -> Vec<bool>;
    fn get_name(&self) -> String;
    fn set_input(&mut self, input: &str, value: bool) -> Result<(), String>;
    // fn get_output(&self, output: &str) -> Result<bool, String>;
}

struct Circuit {
    comput_graphs: Vec<Vec<String>>,
    components: HashMap<String, Box<dyn Chip>>,
    inputs: HashMap<String, bool>,
    outputs: Vec<String>,
    values: Vec<bool>,
    name: String,
}

struct Gate {
    inputs: Vec<(String, bool)>,
    truth_table: Vec<bool>,
    name: String,
}

impl Chip for Gate {
    fn eval(&self) -> Vec<bool> {
        Vec::new()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn set_input(&mut self, input: &str, value: bool) -> Result<(), String> {
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
    fn new(inputs: Vec<(&str, bool)>, truth_table: Vec<bool>, name: &str) -> Result<Self, String> {
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
                .map(|(s, b)| -> (String, bool) { (s.to_string(), b) })
                .collect(),
            name: name.to_string(),
        })
    }
}
