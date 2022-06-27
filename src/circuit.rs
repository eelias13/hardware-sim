use crate::{Error, LookupTable};
use graph::Graph;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub struct InOut {
    name: String,
    value: bool,
}

impl InOut {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: false,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    weight: usize,
    from: usize,
    to: usize,
}

impl Connection {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { weight, from, to }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn from(&self) -> usize {
        self.from
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    Lut(LookupTable),
    In(InOut),
    Out(InOut),
}

pub struct Circuit {
    graph: Graph<Component, Connection>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Component) -> Result<usize, Error> {
        match self.graph.add_node(node.clone()) {
            Ok(value) => {
                match node {
                    Component::In(_) => self.inputs.push(value),
                    Component::Out(_) => self.outputs.push(value),
                    _ => (),
                }
                Ok(value)
            }
            Err(error) => Err(Error::msg(format!("graph error {:?}", error))),
        }
    }

    pub fn add_connection(
        &mut self,
        from: usize,
        to: usize,
        edge: Connection,
    ) -> Result<(), Error> {
        match self.graph.add_edge(from, to, edge) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::msg(format!("graph error {:?}", err))),
        }
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
                Ok(Component::In(node)) => vec![node.value()],
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
                    Ok(Component::Lut(lut)) => lut.set(edge.to(), value[edge.from()])?,
                    Ok(Component::Out(out)) => {
                        if edge.to() == 0 {
                            out.set(value[0])
                        } else {
                            return Err(Error::msg(format!(
                                "unexpected id {} expected 0",
                                edge.to()
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

    pub fn get(&self, node_id: usize) -> Result<bool, Error> {
        if !self.outputs.contains(&node_id) {
            return Err(Error::msg(format!("{} in not an input id", node_id)));
        }

        let node = match self.graph.node(node_id) {
            Err(err) => return Err(Error::msg(format!("graph error {:?}", err))),
            Ok(node) => node.clone(),
        };

        if let Component::Out(node) = node {
            Ok(node.value())
        } else {
            Err(Error::msg(format!("{:?} is not an output node", node)))
        }
    }

    pub fn set(&mut self, node_id: usize, value: bool) -> Result<(), Error> {
        if !self.inputs.contains(&node_id) {
            return Err(Error::msg(format!("{} in not an input id", node_id)));
        }

        let node = match self.graph.node_mut(node_id) {
            Err(err) => return Err(Error::msg(format!("graph error {:?}", err))),
            Ok(node) => node,
        };

        if let Component::In(node) = node {
            node.set(value);
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

    let mut ram = Circuit::new();
    let not_id = ram.add_node(Component::Lut(not)).unwrap();
    let or_id = ram.add_node(Component::Lut(or)).unwrap();
    let and_id = ram.add_node(Component::Lut(and)).unwrap();
    let in_id = ram.add_node(Component::In(InOut::new("input"))).unwrap();
    let out_id = ram.add_node(Component::Out(InOut::new("output"))).unwrap();
    let res_id = ram.add_node(Component::In(InOut::new("reset"))).unwrap();

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

    assert_eq!(ram.get(out_id), Ok(false));

    ram.tick().unwrap();
    ram.set(in_id, true).unwrap();
    assert_eq!(ram.get(out_id), Ok(false));
    ram.tick().unwrap();
    ram.set(in_id, false).unwrap();
    assert_eq!(ram.get(out_id), Ok(true));
    ram.tick().unwrap();
    assert_eq!(ram.get(out_id), Ok(true));
    ram.tick().unwrap();

    ram.set(res_id, true).unwrap();
    ram.tick().unwrap();
    assert_eq!(ram.get(out_id), Ok(false));
    ram.set(res_id, false).unwrap();
    ram.tick().unwrap();
    assert_eq!(ram.get(out_id), Ok(false));
    ram.tick().unwrap();
    assert_eq!(ram.get(out_id), Ok(false));
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

    let mut clock = Circuit::new();
    let not_id = clock.add_node(Component::Lut(not)).unwrap();
    let or_id = clock.add_node(Component::Lut(or)).unwrap();
    let in_id = clock.add_node(Component::In(InOut::new("input"))).unwrap();
    let out_id = clock
        .add_node(Component::Out(InOut::new("output")))
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

    assert_eq!(clock.get(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get(out_id), Ok(true));
    clock.tick().unwrap();
    assert_eq!(clock.get(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get(out_id), Ok(true));
    clock.tick().unwrap();
    assert_eq!(clock.get(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get(out_id), Ok(true));
    clock.tick().unwrap();
    assert_eq!(clock.get(out_id), Ok(false));
    clock.tick().unwrap();
    assert_eq!(clock.get(out_id), Ok(true));
}
