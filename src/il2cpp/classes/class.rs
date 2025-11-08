use crate::il2cpp::classes::{field::Field, method::Method};
use std::fmt::{Debug, Formatter};

pub struct Class {
    pub address: *mut u8,
    pub name: String,
    pub parent: String,
    pub namespace: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}

impl Class {
    pub fn new(address: *mut u8, name: String, parent: String, namespace: String) -> Self {
        Self {
            address,
            name,
            parent,
            namespace,
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class: {:p}\n", self.address)?;
        write!(f, "Name: {}\n", self.name)?;
        write!(f, "Parent: {}\n", self.parent)?;
        write!(f, "Namespace: {}\n", self.namespace)?;
        Ok(())
    }
}
