use super::il2cpp_sys::{
    Il2CppApi, Il2CppAssembly, Il2CppClass, Il2CppDomain, Il2CppImage, Il2CppMethodInfo, Il2CppType,
};
use std::ffi::{CStr, CString};

// A safe wrapper around the raw Il2CppApi
pub struct Il2Cpp<'a> {
    api: &'a Il2CppApi,
}

impl<'a> Il2Cpp<'a> {
    /// Creates a new `Il2Cpp` instance.
    ///
    /// # Safety
    ///
    /// The `api` must be a valid and initialized `Il2CppApi` instance.
    pub unsafe fn new(api: &'a Il2CppApi) -> Self {
        Self { api }
    }

    pub fn get_domain(&self) -> Option<Il2CppDomain> {
        unsafe { self.api.get_domain.map(|f| f()) }
    }

    pub fn thread_attach(&self, domain: Il2CppDomain) {
        if let Some(f) = self.api.thread_attach {
            unsafe { f(domain) };
        }
    }

    pub fn thread_detach(&self, domain: Il2CppDomain) {
        if let Some(f) = self.api.thread_detach {
            unsafe { f(domain) };
        }
    }

    pub fn domain_get_assemblies(&self, domain: Il2CppDomain) -> Vec<Il2CppAssembly> {
        let mut size: usize = 0;
        let mut assemblies = Vec::new();

        if let Some(f) = self.api.domain_get_assemblies {
            unsafe {
                let raw_assemblies = f(domain, &mut size as *mut usize);
                if !raw_assemblies.is_null() && size > 0 {
                    for i in 0..size {
                        let assembly_ptr = *raw_assemblies.add(i);
                        assemblies.push(assembly_ptr);
                    }
                }
            }
        }
        assemblies
    }

    pub fn assembly_get_image(&self, assembly: Il2CppAssembly) -> Option<Il2CppImage> {
        unsafe { self.api.assembly_get_image.map(|f| f(assembly)) }
    }

    pub fn image_get_name(&self, image: Il2CppImage) -> Option<&str> {
        unsafe {
            self.api.image_get_name.and_then(|f| {
                let c_str = f(image);
                if c_str.is_null() {
                    None
                } else {
                    CStr::from_ptr(c_str).to_str().ok()
                }
            })
        }
    }

    pub fn class_from_name(
        &self,
        image: Il2CppImage,
        namespace: &str,
        name: &str,
    ) -> Option<Il2CppClass> {
        let c_namespace = CString::new(namespace).ok()?;
        let c_name = CString::new(name).ok()?;
        unsafe {
            self.api
                .class_from_name
                .map(|f| f(image, c_namespace.as_ptr(), c_name.as_ptr()))
        }
    }

    pub fn class_get_fields(&self, klass: Il2CppClass) -> Vec<*mut u8> {
        let mut iter: usize = 0;
        let mut fields = Vec::new();
        if let Some(f) = self.api.class_get_fields {
            loop {
                let field_ptr = unsafe { f(klass, &mut iter as *mut usize) };
                if field_ptr.is_null() {
                    break;
                }
                fields.push(field_ptr);
            }
        }
        fields
    }

    pub fn field_get_name(&self, field: *mut u8) -> Option<&str> {
        unsafe {
            self.api.field_get_name.and_then(|f| {
                let c_str = f(field);
                if c_str.is_null() {
                    None
                } else {
                    CStr::from_ptr(c_str).to_str().ok()
                }
            })
        }
    }

    pub fn field_get_offset(&self, field: *mut u8) -> Option<u32> {
        unsafe { self.api.field_get_offset.map(|f| f(field)) }
    }

    pub fn class_get_methods(&self, klass: Il2CppClass) -> Vec<Il2CppMethodInfo> {
        let mut iter: usize = 0;
        let mut methods = Vec::new();
        if let Some(f) = self.api.class_get_methods {
            loop {
                let method_ptr = unsafe { f(klass, &mut iter as *mut usize) };
                if method_ptr.is_null() {
                    break;
                }
                methods.push(method_ptr);
            }
        }
        methods
    }

    pub fn method_get_name(&self, method: Il2CppMethodInfo) -> Option<&str> {
        unsafe {
            self.api.method_get_name.and_then(|f| {
                let c_str = f(method);
                if c_str.is_null() {
                    None
                } else {
                    CStr::from_ptr(c_str).to_str().ok()
                }
            })
        }
    }

    pub fn method_get_param_count(&self, method: Il2CppMethodInfo) -> Option<u32> {
        unsafe { self.api.method_get_param_count.map(|f| f(method)) }
    }

    pub fn method_get_param_name(&self, method: Il2CppMethodInfo, index: u32) -> Option<&str> {
        unsafe {
            self.api.method_get_param_name.and_then(|f| {
                let c_str = f(method, index);
                if c_str.is_null() {
                    None
                } else {
                    CStr::from_ptr(c_str).to_str().ok()
                }
            })
        }
    }

    pub fn method_get_return_type(&self, method: Il2CppMethodInfo) -> Option<Il2CppType> {
        unsafe { self.api.method_get_return_type.map(|f| f(method)) }
    }

    pub fn type_get_name(&self, itype: Il2CppType) -> Option<&str> {
        unsafe {
            self.api.type_get_name.and_then(|f| {
                let c_str = f(itype);
                if c_str.is_null() {
                    None
                } else {
                    CStr::from_ptr(c_str).to_str().ok()
                }
            })
        }
    }
}
