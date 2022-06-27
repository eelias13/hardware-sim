use crate::{ChipDef, ComponentDef, Error, LookupTable};
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
    fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { weight, from, to }
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
    pub fn new(
        chip_def: ChipDef<ComponentDef>,
        lut_map: HashMap<String, LookupTable>,
    ) -> Result<Self, Error> {
        let mut circuit = Self::blank(chip_def.name());
        let mut ids = HashMap::new();

        for input in chip_def.inputs() {
            ids.insert(
                input.clone(),
                circuit.add_node(Component::In(InOut::new(input)))?,
            );
        }

        for output in chip_def.outputs() {
            ids.insert(
                output.clone(),
                circuit.add_node(Component::Out(InOut::new(output)))?,
            );
        }

        for part in chip_def.parts() {
            if let Some(lut) = lut_map.get(&part.name()) {
                ids.insert(part.name(), circuit.add_node(Component::Lut(lut.clone()))?);

                for (i, o) in part.inputs() {
                    if let Some(i_id) = lut.in_map(i.clone()) {
                        if let Some(&o_id) = ids.get(&o).clone() {
                            circuit.add_connection(i_id, o_id, Connection::new(i_id, o_id, 1))?;
                            todo!();
                        } else {
                        }
                    } else {
                        return Err(Error::msg(format!(
                            "on table {} no input named {}",
                            lut.name(),
                            i
                        )));
                    }
                }
            } else {
                return Err(Error::msg(format!(
                    "no lookup table found with name {}",
                    part.name()
                )));
            }
        }

        Ok(circuit)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

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

    pub fn get(&self, name: &str) -> Result<bool, Error> {
        if let Some(&id) = self.out_map.get(name).clone() {
            Ok(self.get_id(id)?)
        } else {
            Err(Error::msg(format!("no output named {}", name)))
        }
    }

    pub fn set(&mut self, name: &str, value: bool) -> Result<(), Error> {
        if let Some(&id) = self.in_map.get(name).clone() {
            self.set_id(id, value)
        } else {
            Err(Error::msg(format!("no input named {}", name)))
        }
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

#[test]
fn ram() {
    let not = LookupTable::new(vec![vec![true, false]], vec!["in"], vec!["out"], "not").unwrap();
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

    ram.add_connection(res_id, not_id, Connection::new(0, 0, 1))
        .unwrap();
    ram.add_connection(not_id, and_id, Connection::new(0, 0, 1))
        .unwrap();
    ram.add_connection(and_id, out_id, Connection::new(0, 0, 1))
        .unwrap();
    ram.add_connection(in_id, or_id, Connection::new(0, 1, 1))
        .unwrap();
    ram.add_connection(or_id, and_id, Connection::new(0, 1, 1))
        .unwrap();
    ram.add_connection(and_id, or_id, Connection::new(0, 0, 1))
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
    let not = LookupTable::new(vec![vec![true, false]], vec!["in"], vec!["out"], "not").unwrap();
    let or = LookupTable::new(
        vec![vec![false, true, true, true]],
        vec!["in1", "in2"],
        vec!["out"],
        "or",
    )
    .unwrap();

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
        .add_connection(in_id, or_id, Connection::new(0, 1, 1))
        .unwrap();
    clock
        .add_connection(or_id, not_id, Connection::new(0, 0, 1))
        .unwrap();
    clock
        .add_connection(not_id, out_id, Connection::new(0, 0, 1))
        .unwrap();
    clock
        .add_connection(not_id, or_id, Connection::new(0, 0, 1))
        .unwrap();

    assert_eq!(clock.get_id(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get_id(out_id), Ok(true));
    clock.tick().unwrap();
    assert_eq!(clock.get_id(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get_id(out_id), Ok(true));
    clock.tick().unwrap();
    assert_eq!(clock.get_id(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get_id(out_id), Ok(true));
    clock.tick().unwrap();
    assert_eq!(clock.get_id(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get_id(out_id), Ok(true));
}
