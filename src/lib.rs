// mod chip_def;
// mod circuit;
// mod components;
// mod connection;
// mod entry;
// mod in_out;
// mod lookup_tabel;
//
// pub use chip_def::{ChipDef, ComponentDef, ComponentIO, ComponentMap};
// pub use circuit::Circuit;
// pub use connection::Connection;
// pub use lookup_tabel::LookupTable;
//

#[derive(PartialEq, Debug, Clone)]

pub struct Error {
    msg: String,
}

impl Error {
    pub fn msg(msg: String) -> Self {
        Self { msg: msg }
    }
}

//
// pub trait Component: Clone {
//     fn set(&mut self, in_names: &str, value: bool) -> Result<(), Error>;
//     fn get(&mut self, out_names: &str) -> Result<bool, Error>;
//     fn in_names(&self) -> Vec<String>;
//     fn out_names(&self) -> Vec<String>;
//     fn name(&self) -> String;
//     fn to_lut(&self) -> Option<LookupTable>;
//     fn to_circuit(&self) -> Option<Circuit>;
//     fn set_max_vised(&mut self, max_vised: u8);
// }
//
// trait IODevice: Clone {
//     fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error>;
//     fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error>;
//     fn in_names(&self) -> Vec<String>;
//     fn out_names(&self) -> Vec<String>;
// }
//

use graph::Graph;
mod lookup_tabel;

use lookup_tabel::LookupTable;

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

    pub fn value(&self) -> bool {
        self.value
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
enum Node {
    Lut(LookupTable),
    In(InOut),
    Out(InOut),
}

pub struct Circuit {
    graph: Graph<Node, Connection>,
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

    fn add_node(&mut self, node: Node) -> Result<usize, Error> {
        match self.graph.add_node(node.clone()) {
            Ok(value) => {
                match node {
                    Node::In(_) => self.inputs.push(value),
                    Node::Out(_) => self.outputs.push(value),
                    _ => (),
                }
                Ok(value)
            }
            Err(error) => Err(Error::msg(format!("graph error {:?}", error))),
        }
    }

    fn add_connection(
        &mut self,
        from: usize,
        to: usize,
        edge: Connection,
    ) -> Result<(), graph::Error> {
        self.graph.add_edge(from, to, edge)
    }

    fn tick(&mut self) {}

    fn get(&self, node_id: usize) -> Result<bool, Error> {
        Ok(true)
    }

    fn set(&mut self, node_id: usize) -> Result<(), Error> {
        Ok(())
    }
}

#[test]
fn ram() {}
#[test]
fn clock() {
    let not = LookupTable::new(vec![vec![true, false]], vec!["in"], vec!["out"], "not").unwrap();
    let mut clock = Circuit::new();
    let not_id = clock.add_node(Node::Lut(not)).unwrap();
    let in_id = clock.add_node(Node::In(InOut::new("input"))).unwrap();
    let out_id = clock.add_node(Node::Out(InOut::new("out"))).unwrap();

    clock
        .add_connection(in_id, not_id, Connection::new(0, 0, 1))
        .unwrap();

    clock
        .add_connection(not_id, out_id, Connection::new(0, 0, 1))
        .unwrap();

    clock
        .add_connection(not_id, not_id, Connection::new(0, 0, 1))
        .unwrap();

    assert_eq!(clock.get(out_id), Ok(false));
    clock.tick();
    assert_eq!(clock.get(out_id), Ok(true));
    clock.tick();
    assert_eq!(clock.get(out_id), Ok(false));
    clock.tick();
    assert_eq!(clock.get(out_id), Ok(true));
    clock.tick();
    assert_eq!(clock.get(out_id), Ok(false));
    clock.tick();
    assert_eq!(clock.get(out_id), Ok(true));
}
