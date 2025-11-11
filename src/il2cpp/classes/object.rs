use crate::il2cpp::il2cpp_sys::c_types::Il2CppClass;

#[repr(C)]
#[derive(Debug)]
pub struct MonitorData {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppObject {
    pub klass: Il2CppClass,
    pub monitor: *mut MonitorData,
}

pub type Object = Il2CppObject;
