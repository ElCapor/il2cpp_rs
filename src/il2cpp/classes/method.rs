use crate::il2cpp::classes::arg::Arg;
use crate::il2cpp::classes::class::ClassInner;
use crate::il2cpp::classes::itype::Type;
use std::sync::{Arc, RwLock, Weak};

pub struct MethodInner {
    pub address: *mut u8,
    pub name: String,
    pub class: Weak<ClassInner>,
    pub return_type: Type,
    pub flags: i32,
    pub static_methodon: bool,
    pub function: *mut u8,
    pub args: RwLock<Vec<Arg>>,
}

pub type Method = Arc<MethodInner>;

impl MethodInner {
    pub fn new(
        address: *mut u8,
        name: String,
        class: Weak<ClassInner>,
        return_type: Type,
        flags: i32,
        static_methodon: bool,
        function: *mut u8,
        args: RwLock<Vec<Arg>>,
    ) -> Method {
        Arc::new(Self {
            address,
            name,
            class,
            return_type,
            flags,
            static_methodon,
            function,
            args,
        })
    }
}
