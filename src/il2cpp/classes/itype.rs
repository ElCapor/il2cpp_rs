use std::sync::Arc;

use crate::il2cpp::il2cpp_sys::c_types::Il2CppType;

#[derive(Debug)]
pub struct TypeInner {
    pub address: Il2CppType,
    pub name: String,
    pub size: isize,
}

// Public handle used throughout the model
pub type Type = Arc<TypeInner>;

impl TypeInner {
    pub fn default() -> Type {
        Arc::new(Self {
            address: std::ptr::null_mut(),
            name: "".to_string(),
            size: 0,
        })
    }

    pub fn new(address: Il2CppType, name: String, size: isize) -> Type {
        Arc::new(Self {
            address,
            name,
            size,
        })
    }
}

unsafe impl Send for TypeInner {}
unsafe impl Sync for TypeInner {}
