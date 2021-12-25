use crate::{Component, Connection, Error, IODevice};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Entry<T: Component> {
    viseted: u8,
    component: Box<T>,
    in_map: HashMap<String, String>,
    out_map: HashMap<String, String>,
}

impl<T: Component> Entry<T> {
    pub fn new(component: Box<T>, connection: Connection) -> Result<Self, Error> {
        // TODO validate
        Ok(Self {
            viseted: 0,
            component,
            in_map: connection.in_map(),
            out_map: connection.out_map(),
        })
    }
}

impl<T: Component> IODevice for Entry<T> {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error> {
        if self.viseted > max_vised {
            // TODO
            todo!();
        }
        if let Some(name) = self.in_map.get(in_name) {
            self.viseted += 1;
            self.component.as_mut().set(name, value)
        } else {
            // TODO
            todo!();
        }
    }
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error> {
        if self.viseted > max_vised {
            // TODO
            todo!();
        }
        if let Some(name) = self.out_map.get(out_name) {
            self.viseted += 1;
            self.component.as_mut().get(name)
        } else {
            // TODO
            todo!();
        }
    }

    fn in_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for key in self.in_map.keys() {
            result.push(key.clone());
        }
        result
    }
    fn out_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for key in self.out_map.keys() {
            result.push(key.clone());
        }
        result
    }
}
