use crate::{ChipDef, Error, LookupTable};
use bool_algebra::update_values;
use graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
struct InOut {
    name: String,
    value: bool,
}

impl InOut {
    fn new(name: String) -> Self {
        Self { name, value: false }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Connection {
    weight: usize,
    from: usize,
    to: usize,
}

impl Connection {
    fn new(from: usize, to: usize) -> Self {
        Self {
            weight: 1,
            from,
            to,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Component {
    Lut(LookupTable),
    In(InOut),
    Out(InOut),
}

pub struct Circuit {
    name: String,
    graph: Graph<Component, Connection>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    in_map: HashMap<String, usize>,
    out_map: HashMap<String, usize>,
}

impl Circuit {
    /// creates a new `Circuit` form the `ChipDef`
    pub fn new(chip_def: ChipDef, lut_map: HashMap<String, LookupTable>) -> Result<Self, Error> {
        let mut circuit = Self::blank(chip_def.name());
        // maps lut name with the corisponding node id in the circuit graph
        let mut ids = HashMap::new();
        // maps the internal connection name the the corisponding (lut id, port id)
        let mut in_ports = HashMap::new();

        for input in chip_def.inputs() {
            let lut_id = circuit.add_node(Component::In(InOut::new(input.clone())))?;
            ids.insert(input.clone(), lut_id);
        }

        for output in chip_def.outputs() {
            let lut_id = circuit.add_node(Component::Out(InOut::new(output.clone())))?;
            ids.insert(output.clone(), lut_id);
            in_ports.insert(output.clone(), vec![(lut_id, 0)]);
        }

        // initilse in_ports and add lut as nodes
        for part in chip_def.parts() {
            if let crate::Component::Def {
                inputs,
                outputs: _,
                name,
            } = part
            {
                if let Some(lut) = lut_map.get(&name) {
                    let lut_id = circuit.add_node(Component::Lut(lut.clone()))?;
                    ids.insert(name, lut_id);
                    if inputs.len() != lut.in_names().len() {
                        return Err(Error::msg("wrong size".to_string()));
                    }

                    for (i, o) in inputs {
                        if let Some(v) = in_ports.get_mut(&o) {
                            v.push((lut_id, lut.in_map(&i).unwrap()));
                        } else {
                            in_ports.insert(o, vec![(lut_id, lut.in_map(&i).unwrap())]);
                        }
                    }
                } else {
                    return Err(Error::msg(format!(
                        "no lookup table found with name {}",
                        name
                    )));
                }
            } else {
            }
        }

        // create conections
        for part in chip_def.parts() {
            if let crate::Component::Def {
                inputs: _,
                outputs,
                name,
            } = part
            {
                for (i, o) in outputs {
                    let from_lut_id = circuit.lut_name(&name)?;
                    let from_port = circuit.lut_id(from_lut_id)?.out_map(&i).unwrap();
                    for (to_lut_id, to_port) in in_ports.get(&o).unwrap().clone() {
                        circuit.add_connection(
                            from_lut_id,
                            to_lut_id,
                            Connection::new(from_port, to_port),
                        )?;
                    }
                }
            } else {
            }
        }

        // connecting io inputs to lut
        for input in chip_def.inputs() {
            if let Some(&in_id) = ids.get(&input).clone() {
                for (to_lut_id, to_port) in in_ports.get(&input).unwrap().clone() {
                    circuit.add_connection(in_id, to_lut_id, Connection::new(0, to_port))?;
                }
            }
        }

        Ok(circuit)
    }

    /// updates the `Circuit`
    pub fn tick(&mut self) -> Result<(), Error> {
        let mut queue = VecDeque::with_capacity(self.inputs.len());
        let mut viseted = HashSet::new();

        for input in self.inputs.clone() {
            queue.push_back(input);
        }

        while let Some(node_id) = queue.pop_front() {
            viseted.insert(node_id);

            let value = match self.graph.node(node_id) {
                Ok(Component::In(node)) => vec![node.value],
                Ok(Component::Lut(lut)) => lut.outputs(),
                Ok(Component::Out(_)) => continue,
                Err(err) => return Err(Error::msg(format!("graph error {:?}", err))),
            };

            for (edge, id) in match self.graph.out_edges(node_id) {
                Ok(vec) => vec,
                Err(err) => return Err(Error::msg(format!("graph error {:?}", err))),
            } {
                if viseted.get(&id) == None {
                    queue.push_back(id);
                }

                match self.graph.node_mut(id) {
                    Ok(Component::Lut(lut)) => lut.set_id(edge.to, value[edge.from])?,
                    Ok(Component::Out(out)) => {
                        if edge.to == 0 {
                            out.value = value[0]
                        } else {
                            return Err(Error::msg(format!(
                                "unexpected id {} expected 0",
                                edge.to
                            )));
                        }
                    }
                    Ok(node) => return Err(Error::msg(format!("unexpected type {:?}", node))),
                    Err(err) => return Err(Error::msg(format!("graph error {:?}", err))),
                }
            }
        }
        Ok(())
    }

    /// gets the output named `name` and returs the value
    /// returs an error if the name dose not exist
    pub fn get(&self, name: &str) -> Result<bool, Error> {
        if let Some(&id) = self.out_map.get(name).clone() {
            Ok(self.get_id(id)?)
        } else {
            Err(Error::msg(format!("no output named {}", name)))
        }
    }

    /// sets the input named `name` to the value `value`
    /// returs an error if the name dose not exist
    pub fn set(&mut self, name: &str, value: bool) -> Result<(), Error> {
        if let Some(&id) = self.in_map.get(name).clone() {
            self.set_id(id, value)
        } else {
            Err(Error::msg(format!("no input named {}", name)))
        }
    }

    /// retuns the name of the `Circuit`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// trys to transform the `Circuit` in to a `LookupTable`
    pub fn to_lut(mut self) -> Result<Option<LookupTable>, Error> {
        if !self.graph.is_dag() {
            return Ok(None);
        }

        let mut inputs = vec![false; self.inputs.len()];
        let mut table =
            vec![Vec::with_capacity(2_usize.pow(inputs.len() as u32)); self.outputs.len()];

        let in_names = self.in_names();
        let out_names = self.out_names();

        // runs for 2_usize.pow(inputs.len() as u32)
        loop {
            for i in 0..inputs.len() {
                self.set(&in_names[i], inputs[i])?;
            }
            self.tick()?;

            for i in 0..table.len() {
                table[i].push(self.get(&out_names[i])?);
            }
            if !update_values(&mut inputs) {
                break;
            }
        }

        Ok(Some(LookupTable::new(
            table,
            in_names.iter().map(|s| s.as_str()).collect(),
            out_names.iter().map(|s| s.as_str()).collect(),
            self.name.as_str(),
        )?))
    }

    /// gest names of inputs
    /// note the order can cange if you add new inputs
    pub fn in_names(&self) -> Vec<String> {
        self.in_map
            .clone()
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// gest names of outputs
    /// note the order can cange if you add new outputs
    pub fn out_names(&self) -> Vec<String> {
        self.out_map
            .clone()
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    }
}

impl Circuit {
    fn blank(name: String) -> Self {
        Self {
            name,
            graph: Graph::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            in_map: HashMap::new(),
            out_map: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Component) -> Result<usize, Error> {
        match self.graph.add_node(node.clone()) {
            Ok(value) => {
                match node {
                    Component::In(node) => {
                        self.inputs.push(value);
                        self.in_map.insert(node.name, value);
                    }
                    Component::Out(node) => {
                        self.outputs.push(value);
                        self.out_map.insert(node.name, value);
                    }
                    _ => (),
                }
                Ok(value)
            }
            Err(error) => Err(Error::msg(format!("graph error {:?}", error))),
        }
    }

    fn lut_name(&self, name: &String) -> Result<usize, Error> {
        for (id, node) in self.graph.nodes().into_iter().enumerate() {
            if let Component::Lut(lut) = node {
                if &lut.name() == name {
                    return Ok(id);
                }
            }
        }
        Err(Error::msg(format!("lut {} not found", name)))
    }

    fn lut_id(&self, node_id: usize) -> Result<LookupTable, Error> {
        if let Ok(Component::Lut(lut)) = self.graph.node(node_id) {
            Ok(lut)
        } else {
            Err(Error::msg(format!("lut {} not found", node_id)))
        }
    }

    fn add_connection(&mut self, from: usize, to: usize, edge: Connection) -> Result<(), Error> {
        match self.graph.add_edge(from, to, edge) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::msg(format!("graph error {:?}", err))),
        }
    }

    fn get_id(&self, node_id: usize) -> Result<bool, Error> {
        if !self.outputs.contains(&node_id) {
            return Err(Error::msg(format!("{} in not an input id", node_id)));
        }

        let node = match self.graph.node(node_id) {
            Err(err) => return Err(Error::msg(format!("graph error {:?}", err))),
            Ok(node) => node.clone(),
        };

        if let Component::Out(node) = node {
            Ok(node.value)
        } else {
            Err(Error::msg(format!("{:?} is not an output node", node)))
        }
    }

    fn set_id(&mut self, node_id: usize, value: bool) -> Result<(), Error> {
        if !self.inputs.contains(&node_id) {
            return Err(Error::msg(format!("{} in not an input id", node_id)));
        }

        let node = match self.graph.node_mut(node_id) {
            Err(err) => return Err(Error::msg(format!("graph error {:?}", err))),
            Ok(node) => node,
        };

        if let Component::In(node) = node {
            node.value = value;
            Ok(())
        } else {
            Err(Error::msg(format!("{:?} is not an input node", node)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit;

    use super::*;

    #[test]
    fn ram() {
        let not =
            LookupTable::new(vec![vec![true, false]], vec!["in"], vec!["out"], "not").unwrap();
        let and = LookupTable::new(
            vec![vec![false, false, false, true]],
            vec!["in1", "in2"],
            vec!["out"],
            "and",
        )
        .unwrap();
        let or = LookupTable::new(
            vec![vec![false, true, true, true]],
            vec!["in1", "in2"],
            vec!["out"],
            "or",
        )
        .unwrap();

        let mut ram = Circuit::blank("ram".to_string());
        let not_id = ram.add_node(Component::Lut(not)).unwrap();
        let or_id = ram.add_node(Component::Lut(or)).unwrap();
        let and_id = ram.add_node(Component::Lut(and)).unwrap();
        let in_id = ram
            .add_node(Component::In(InOut::new("input".to_string())))
            .unwrap();
        let out_id = ram
            .add_node(Component::Out(InOut::new("output".to_string())))
            .unwrap();
        let res_id = ram
            .add_node(Component::In(InOut::new("reset".to_string())))
            .unwrap();

        ram.add_connection(res_id, not_id, Connection::new(0, 0))
            .unwrap();
        ram.add_connection(not_id, and_id, Connection::new(0, 0))
            .unwrap();
        ram.add_connection(and_id, out_id, Connection::new(0, 0))
            .unwrap();
        ram.add_connection(in_id, or_id, Connection::new(0, 1))
            .unwrap();
        ram.add_connection(or_id, and_id, Connection::new(0, 1))
            .unwrap();
        ram.add_connection(and_id, or_id, Connection::new(0, 0))
            .unwrap();

        assert_eq!(ram.get_id(out_id), Ok(false));

        ram.tick().unwrap();
        ram.set_id(in_id, true).unwrap();
        assert_eq!(ram.get_id(out_id), Ok(false));
        ram.tick().unwrap();
        ram.set_id(in_id, false).unwrap();
        assert_eq!(ram.get_id(out_id), Ok(true));
        ram.tick().unwrap();
        assert_eq!(ram.get_id(out_id), Ok(true));
        ram.tick().unwrap();

        ram.set_id(res_id, true).unwrap();
        ram.tick().unwrap();
        assert_eq!(ram.get_id(out_id), Ok(false));
        ram.set_id(res_id, false).unwrap();
        ram.tick().unwrap();
        assert_eq!(ram.get_id(out_id), Ok(false));
        ram.tick().unwrap();
        assert_eq!(ram.get_id(out_id), Ok(false));
        ram.tick().unwrap();
    }

    #[test]
    fn clock() {
        let chip_def = ChipDef::new(
            "clock",
            vec!["input"],
            vec!["output"],
            vec![
                (vec![("in", "in")], vec![("out", "output")], "not"),
                (
                    vec![("in1", "input"), ("in2", "output")],
                    vec![("out", "in")],
                    "or",
                ),
            ],
        );

        let not =
            LookupTable::new(vec![vec![true, false]], vec!["in"], vec!["out"], "not").unwrap();
        let or = LookupTable::new(
            vec![vec![false, true, true, true]],
            vec!["in1", "in2"],
            vec!["out"],
            "or",
        )
        .unwrap();

        let mut lut_map = HashMap::new();

        lut_map.insert("not".to_string(), not.clone());
        lut_map.insert("or".to_string(), or.clone());

        let circuit = Circuit::new(chip_def, lut_map).unwrap();

        let mut clock = Circuit::blank("clock".to_string());
        let not_id = clock.add_node(Component::Lut(not)).unwrap();
        let or_id = clock.add_node(Component::Lut(or)).unwrap();
        let in_id = clock
            .add_node(Component::In(InOut::new("input".to_string())))
            .unwrap();
        let out_id = clock
            .add_node(Component::Out(InOut::new("output".to_string())))
            .unwrap();

        clock
            .add_connection(in_id, or_id, Connection::new(0, 1))
            .unwrap();
        clock
            .add_connection(or_id, not_id, Connection::new(0, 0))
            .unwrap();
        clock
            .add_connection(not_id, out_id, Connection::new(0, 0))
            .unwrap();
        clock
            .add_connection(not_id, or_id, Connection::new(0, 0))
            .unwrap();

        assert_eq!(clock.get_id(out_id), Ok(false));
        assert_eq!(clock.get("output"), Ok(false));
        clock.tick().unwrap();
        assert_eq!(clock.get("output"), Ok(true));
        assert_eq!(clock.get_id(out_id), Ok(true));
        clock.tick().unwrap();
        assert_eq!(clock.get("output"), Ok(false));
        assert_eq!(clock.get_id(out_id), Ok(false));
        clock.tick().unwrap();
        assert_eq!(clock.get("output"), Ok(true));
        assert_eq!(clock.get_id(out_id), Ok(true));
        clock.tick().unwrap();
        assert_eq!(clock.get("output"), Ok(false));
        assert_eq!(clock.get_id(out_id), Ok(false));
        clock.tick().unwrap();
        assert_eq!(clock.get("output"), Ok(true));
        assert_eq!(clock.get_id(out_id), Ok(true));
        clock.tick().unwrap();
        assert_eq!(clock.get("output"), Ok(false));
        assert_eq!(clock.get_id(out_id), Ok(false));
        clock.tick().unwrap();
        assert_eq!(clock.get("output"), Ok(true));
        assert_eq!(clock.get_id(out_id), Ok(true));

        assert_eq!(circuit.graph.edge_list(), clock.graph.edge_list());
    }

    #[test]
    fn and_graph() {
        let lut = LookupTable::new(
            vec![vec![false, false, false, true]],
            vec!["a", "b"],
            vec!["out"],
            "And",
        )
        .unwrap();

        let def = ChipDef::new(
            "And",
            vec!["a", "b"],
            vec!["out"],
            vec![(vec![("a", "a"), ("b", "b")], vec![("out", "out")], "And")],
        );

        let mut map = HashMap::new();
        map.insert("And".to_string(), lut.clone());

        let circuit_from_def = Circuit::new(def, map).unwrap();

        let mut circuit = Circuit::blank("And".to_string());
        let in1 = circuit
            .add_node(Component::In(InOut::new("".to_string())))
            .unwrap();
        let in2 = circuit
            .add_node(Component::In(InOut::new("".to_string())))
            .unwrap();
        let output = circuit
            .add_node(Component::Out(InOut::new("out".to_string())))
            .unwrap();
        let lut = circuit.add_node(Component::Lut(lut)).unwrap();

        circuit
            .add_connection(in1, lut, Connection::new(0, 0))
            .unwrap();
        circuit
            .add_connection(in2, lut, Connection::new(0, 1))
            .unwrap();
        circuit
            .add_connection(lut, output, Connection::new(0, 0))
            .unwrap();

        assert_eq!(
            circuit.graph.edge_list(),
            circuit_from_def.graph.edge_list()
        );
    }

    #[test]
    fn not() {
        let lut = LookupTable::new(
            vec![vec![true, true, true, false]],
            vec!["a", "b"],
            vec!["out"],
            "Nand",
        )
        .unwrap();

        let mut lut_map = HashMap::new();
        lut_map.insert("Nand".to_string(), lut.clone());

        let def = ChipDef::new(
            "Not",
            vec!["input"],
            vec!["output"],
            vec![(
                vec![("a", "input"), ("b", "input")],
                vec![("out", "output")],
                "Nand",
            )],
        );

        let circuit_from_def = Circuit::new(def, lut_map).unwrap();

        let mut circuit = Circuit::blank("Not".to_string());
        let input = circuit
            .add_node(Component::In(InOut::new("input".to_string())))
            .unwrap();
        let output = circuit
            .add_node(Component::Out(InOut::new("output".to_string())))
            .unwrap();
        let lut = circuit.add_node(Component::Lut(lut)).unwrap();

        circuit
            .add_connection(input, lut, Connection::new(0, 0))
            .unwrap();
        circuit
            .add_connection(input, lut, Connection::new(0, 1))
            .unwrap();
        circuit
            .add_connection(lut, output, Connection::new(0, 0))
            .unwrap();

        assert_eq!(circuit.tick(), Ok(()));
        assert_eq!(circuit.get("output"), Ok(true));

        assert_eq!(circuit.set("input", true), Ok(()));
        assert_eq!(circuit.tick(), Ok(()));
        assert_eq!(circuit.get("output"), Ok(false));

        assert_eq!(
            circuit.graph.edge_list(),
            circuit_from_def.graph.edge_list()
        );
    }

    #[test]
    fn and_from_nand() {
        let lut = LookupTable::new(
            vec![vec![true, true, true, false]],
            vec!["a", "b"],
            vec!["out"],
            "Nand",
        )
        .unwrap();

        let mut lut_map = HashMap::new();
        lut_map.insert("Nand".to_string(), lut.clone());

        let def = ChipDef::new(
            "And",
            vec!["a", "b"],
            vec!["out"],
            vec![
                (vec![("a", "a"), ("b", "b")], vec![("out", "nand")], "Nand"),
                (
                    vec![("a", "nand"), ("b", "nand")],
                    vec![("out", "out")],
                    "Nand",
                ),
            ],
        );

        let circuit_from_def = Circuit::new(def, lut_map).unwrap();

        let mut circuit = Circuit::blank("And".to_string());

        let a = circuit
            .add_node(Component::In(InOut::new("a".to_string())))
            .unwrap();
        let b = circuit
            .add_node(Component::In(InOut::new("b".to_string())))
            .unwrap();
        let out = circuit
            .add_node(Component::Out(InOut::new("out".to_string())))
            .unwrap();
        let nand = circuit.add_node(Component::Lut(lut.clone())).unwrap();
        let not = circuit.add_node(Component::Lut(lut.clone())).unwrap();

        circuit
            .add_connection(a, nand, Connection::new(0, 0))
            .unwrap();
        circuit
            .add_connection(b, nand, Connection::new(0, 1))
            .unwrap();
        circuit
            .add_connection(nand, not, Connection::new(0, 0))
            .unwrap();
        circuit
            .add_connection(nand, not, Connection::new(0, 1))
            .unwrap();
        circuit
            .add_connection(not, out, Connection::new(0, 0))
            .unwrap();

        assert_eq!(circuit.tick(), Ok(()));
        assert_eq!(circuit.get("out"), Ok(false));
        assert_eq!(circuit.set("a", true), Ok(()));
        assert_eq!(circuit.tick(), Ok(()));
        assert_eq!(circuit.get("out"), Ok(false));
        assert_eq!(circuit.set("b", true), Ok(()));
        assert_eq!(circuit.tick(), Ok(()));
        assert_eq!(circuit.get("out"), Ok(true));
        assert_eq!(circuit.set("a", false), Ok(()));
        assert_eq!(circuit.tick(), Ok(()));
        assert_eq!(circuit.get("out"), Ok(false));

        assert_eq!(circuit.graph.is_dag(), true);

        assert_eq!(
            circuit.graph.edge_list(),
            circuit_from_def.graph.edge_list()
        );
    }
}
