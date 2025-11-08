use crate::il2cpp::classes::itype::Type;

use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Field {
    address: *mut u8,
    name: String,
    itype: Type,
    class: *const u8,
    offset: i32,
    static_field: bool,
    vtable: *mut u8,
}

impl Field {
    pub fn new(
        address: *mut u8,
        name: String,
        itype: Type,
        class: *mut u8,
        offset: i32,
        static_field: bool,
        vtable: *mut u8,
    ) -> Self {
        Self {
            address,
            name,
            itype,
            class,
            offset,
            static_field,
            vtable,
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field: {:p}\n", self.address)?;
        write!(f, "Name: {}\n", self.name)?;
        //write!(f, "Type: {:p}\n", self.itype.address)?;
        write!(f, "Class: {:p}\n", self.class)?;
        write!(f, "Offset: {}\n", self.offset)?;
        write!(f, "StaticField: {}\n", self.static_field)?;
        write!(f, "Vtable: {:p}\n", self.vtable)?;
        Ok(())
    }
}
