use crate::il2cpp::classes::{field::Field, method::Method};
use parking_lot::RwLock;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct ClassInner {
    pub address: *mut u8,
    pub name: String,
    pub parent: String,
    pub namespace: String,
    pub fields: RwLock<Vec<Field>>,
    pub methods: RwLock<Vec<Method>>,
}

pub type Class = Arc<ClassInner>;

impl ClassInner {
    pub fn new(address: *mut u8, name: String, parent: String, namespace: String) -> Class {
        Arc::new(Self {
            address,
            name,
            parent,
            namespace,
            fields: RwLock::new(Vec::new()),
            methods: RwLock::new(Vec::new()),
        })
    }

    pub fn get_field(&self, name: &str) -> Option<Field> {
        let guard = self.fields.read();
        guard.iter().find(|f| f.name == name).cloned()
    }

    pub fn get_method(&self, name: &str, args: Vec<&str>) -> Option<Method> {
        let guard = self.methods.read();
        guard
            .iter()
            .find(|m| {
                if m.name.trim() == name.trim() {
                    if m.args.read().is_empty() && args.is_empty() {
                        return true;
                    } else if m.args.read().len() == args.len() {
                        for (arg, arg2) in m.args.read().iter().zip(args.iter()) {
                            if arg.itype.name != arg2.to_string() {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    false
                }
            })
            .cloned()
    }
}

impl Debug for ClassInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class: {:p}\n", self.address)?;
        write!(f, "Name: {}\n", self.name)?;
        write!(f, "Parent: {}\n", self.parent)?;
        write!(f, "Namespace: {}\n", self.namespace)?;
        let fields = self.fields.read();
        write!(f, "Fields Len: {}\n", fields.len())?;
        for field in fields.iter() {
            write!(f, "{:?}", field)?;
        }
        Ok(())
    }
}

unsafe impl Send for ClassInner {}
unsafe impl Sync for ClassInner {}
