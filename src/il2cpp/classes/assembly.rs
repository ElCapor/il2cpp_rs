use super::class::Class;
use std::fmt::{Debug, Formatter};
pub struct Assembly {
    pub address: *mut u8,
    pub name: String,
    pub file: String,
    pub classes: Vec<Class>,
}

impl Assembly {
    pub fn new(address: *mut u8, name: String, file: String) -> Self {
        Self {
            address,
            name,
            file,
            classes: Vec::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.name == name)
    }
}

impl Debug for Assembly {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assembly: {:p}\n", self.address)?;
        write!(f, "Name: {}\n", self.name)?;
        write!(f, "File: {}\n", self.file)?;
        write!(f, "Classes Len: {}\n", self.classes.len())?;
        for class in &self.classes {
            write!(f, "{:?}", class)?;
        }
        Ok(())
    }
}

unsafe impl Send for Assembly {}
unsafe impl Sync for Assembly {}
