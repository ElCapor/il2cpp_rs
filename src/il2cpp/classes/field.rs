use crate::il2cpp::classes::class::ClassInner;
use crate::il2cpp::classes::itype::Type;

use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Weak};

pub struct FieldInner {
    pub address: *mut u8,
    pub name: String,
    pub itype: Type,
    pub class: Weak<ClassInner>,
    pub offset: i32,
    pub static_field: bool,
    pub vtable: *mut u8,
}

pub type Field = Arc<FieldInner>;

impl FieldInner {
    pub fn new(
        address: *mut u8,
        name: String,
        itype: Type,
        class: Weak<ClassInner>,
        offset: i32,
        static_field: bool,
        vtable: *mut u8,
    ) -> Field {
        Arc::new(Self {
            address,
            name,
            itype,
            class,
            offset,
            static_field,
            vtable,
        })
    }
}

impl Debug for FieldInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field: {:p}\n", self.address)?;
        write!(f, "Name: {}\n", self.name)?;
        if let Some(cls) = self.class.upgrade() {
            write!(f, "Class: {:p}\n", cls.address)?;
        } else {
            write!(f, "Class: <dropped>\n")?;
        }
        write!(f, "Offset: {}\n", self.offset)?;
        write!(f, "StaticField: {}\n", self.static_field)?;
        write!(f, "Vtable: {:p}\n", self.vtable)?;
        Ok(())
    }
}
