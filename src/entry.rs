use crate::{Component, Connection, Error, IODevice};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Entry<T: Component> {
    viseted: u8,
    component: T,
    in_map: HashMap<String, String>,
    out_map: HashMap<String, String>,
}

impl<T: Component> Entry<T> {
    pub fn new(component: T, connection: Connection) -> Result<Self, Error> {
        // TODO validate
        Ok(Self {
            viseted: 0,
            component,
            in_map: connection.in_map(),
            out_map: connection.out_map(),
        })
    }

    pub fn set_max_vised(&mut self, max_vised: u8) {
        self.component.set_max_vised(max_vised);
    }

    pub fn name(&self) -> String {
        self.component.name()
    }
}

impl<T: Component> IODevice for Entry<T> {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error> {
        if self.viseted > max_vised {
            Err(Error::msg(format!(
                "in_name {} not found for component {}",
                in_name,
                self.component.name()
            )))
        } else if let Some(name) = self.in_map.get(in_name) {
            self.viseted += 1;
            self.component.set(name, value)
        } else {
            Err(Error::msg(format!("in_name {} not found", in_name)))
        }
    }
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error> {
        if self.viseted > max_vised {
            Err(Error::msg(format!("max_vised {} reached", max_vised)))
        } else if let Some(name) = self.out_map.get(out_name) {
            self.viseted += 1;
            self.component.get(name)
        } else {
            Err(Error::msg(format!(
                "out_name {} not found for component {}",
                out_name,
                self.component.name()
            )))
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
