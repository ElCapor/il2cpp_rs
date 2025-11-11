use crate::il2cpp::classes::object::Object;

#[repr(C)]
pub struct UnityObject {
    pub obj: Object, // inherited fields
    pub m_cached_ptr: *mut std::ffi::c_void,
}

impl UnityObject {
    /// Create a reference from a raw pointer returned by IL2CPP
    pub fn from_ptr<'a>(ptr: *mut UnityObject) -> Option<&'a Self> {
        unsafe { std::ptr::NonNull::new(ptr).map(|nn| nn.as_ref()) }
    }

    /// Mutable version
    pub fn from_ptr_mut<'a>(ptr: *mut UnityObject) -> Option<&'a mut Self> {
        unsafe { std::ptr::NonNull::new(ptr).map(|mut nn| nn.as_mut()) }
    }

    /// Get the native cached pointer
    pub fn cached_ptr(&self) -> *mut std::ffi::c_void {
        self.m_cached_ptr
    }
}
