use crate::{Error, IODevice};

#[derive(Debug, Clone, PartialEq)]
pub struct InOut {
    name: String,
    viseted: u8,
    value: bool,
}

impl InOut {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            viseted: 0,
            value: false,
        }
    }
}

impl IODevice for InOut {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error> {
        if in_name != self.name {
            Err(Error::msg(format!(
                "wrong in_name expected {} but got {} ",
                self.name, in_name
            )))
        } else if self.viseted > max_vised {
            Err(Error::msg(format!("max_vised {} reached", max_vised)))
        } else {
            self.viseted += 1;
            self.value = value;
            Ok(())
        }
    }
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error> {
        if out_name != self.name {
            Err(Error::msg(format!(
                "wrong out_name expected {} but got {} ",
                self.name, out_name
            )))
        } else if self.viseted > max_vised {
            Err(Error::msg(format!("max_vised {} reached", max_vised)))
        } else {
            self.viseted += 1;
            Ok(self.value)
        }
    }
    fn in_names(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
    fn out_names(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
