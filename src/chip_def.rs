#[derive(Debug, Clone, PartialEq)]
pub struct ChipDef<T> {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    parts: Vec<T>,
}

impl<T> ChipDef<T> {
    pub fn new(name: &str, inputs: Vec<&str>, outputs: Vec<&str>, parts: Vec<T>) -> Self {
        Self {
            name: name.to_string(),
            inputs: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            outputs: outputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            parts,
        }
    }

    pub fn new_string(
        name: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        parts: Vec<T>,
    ) -> Self {
        Self {
            name,
            inputs,
            outputs,
            parts,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMap {
    var_map: Vec<(String, String)>,
    name: String,
}

impl ComponentMap {
    pub fn new(var_map: Vec<(&str, &str)>, name: &str) -> Self {
        Self {
            var_map: var_map
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),
            name: name.to_string(),
        }
    }

    pub fn new_string(var_map: Vec<(String, String)>, name: String) -> Self {
        Self { var_map, name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentIO {
    inputs: Vec<String>,
    ouputs: Vec<String>,
    name: String,
}

impl ComponentIO {
    pub fn new(inputs: Vec<&str>, ouputs: Vec<&str>, name: &str) -> Self {
        Self {
            inputs: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            ouputs: ouputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentDef {
    inputs: Vec<(String, String)>,
    ouputs: Vec<(String, String)>,
    name: String,
}

impl ComponentDef {
    pub fn new(inputs: Vec<(&str, &str)>, ouputs: Vec<(&str, &str)>, name: &str) -> Self {
        Self {
            inputs: inputs
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),

            ouputs: ouputs
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),
            name: name.to_string(),
        }
    }
}
