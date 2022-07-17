#[derive(Debug, Clone, PartialEq)]
pub struct ChipDef {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    parts: Vec<Component>,
}

impl ChipDef {
    pub fn new(
        name: &str,
        inputs: Vec<&str>,
        outputs: Vec<&str>,
        parts: Vec<(Vec<(&str, &str)>, Vec<(&str, &str)>, &str)>,
    ) -> Self {
        Self {
            name: name.to_string(),
            inputs: inputs.iter().map(|&s| s.to_string()).collect(),
            outputs: outputs.iter().map(|&s| s.to_string()).collect(),
            parts: parts
                .iter()
                .map(|(inputs, outputs, name)| Component::Def {
                    inputs: inputs
                        .iter()
                        .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                        .collect(),
                    outputs: outputs
                        .iter()
                        .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                        .collect(),
                    name: name.to_string(),
                })
                .collect(),
        }
    }

    pub fn new_string(
        name: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        parts: Vec<Component>,
    ) -> Self {
        Self {
            name,
            inputs,
            outputs,
            parts,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn inputs(&self) -> Vec<String> {
        self.inputs.clone()
    }

    pub fn outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }

    pub fn parts(&self) -> Vec<Component> {
        self.parts.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    Map {
        var_map: Vec<(String, String)>,
        name: String,
    },

    IO {
        inputs: Vec<String>,
        outputs: Vec<String>,
        name: String,
    },

    Def {
        inputs: Vec<(String, String)>,
        outputs: Vec<(String, String)>,
        name: String,
    },
}

impl Component {
    pub fn name(&self) -> String {
        match self {
            Self::Map { var_map: _, name } => name.clone(),

            Self::IO {
                inputs: _,
                outputs: _,
                name,
            } => name.clone(),

            Self::Def {
                inputs: _,
                outputs: _,
                name,
            } => name.clone(),
        }
    }
}
