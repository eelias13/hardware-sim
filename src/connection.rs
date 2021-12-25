use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    in_map: HashMap<String, String>,
    out_map: HashMap<String, String>,
    comp_name: String,
}

impl Connection {
    pub fn new(comp_name: &str, in_tuple: Vec<(&str, &str)>, out_tuple: Vec<(&str, &str)>) -> Self {
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

    pub fn in_map(&self) -> HashMap<String, String> {
        self.in_map.clone()
    }

    pub fn out_map(&self) -> HashMap<String, String> {
        self.out_map.clone()
    }

    pub fn name(&self) -> String {
        self.comp_name.clone()
    }
}
