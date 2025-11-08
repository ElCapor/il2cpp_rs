pub mod classes;
pub mod il2cpp_sys;

use il2cpp_sys::c_types::{
    Il2CppAssembly, Il2CppClass, Il2CppDomain, Il2CppImage, Il2CppMethodInfo, Il2CppType,
};

use std::ffi::{CStr, CString};

pub fn get_domain() -> Result<Il2CppDomain, String> {
    il2cpp_sys::il2cpp_domain_get()
}

pub fn thread_attach(domain: Il2CppDomain) -> Result<(), String> {
    il2cpp_sys::il2cpp_thread_attach(domain)
}

pub fn thread_detach(domain: Il2CppDomain) -> Result<(), String> {
    il2cpp_sys::il2cpp_thread_detach(domain)
}

pub fn domain_get_assemblies(domain: Il2CppDomain) -> Result<Vec<Il2CppAssembly>, String> {
    let mut size: usize = 0;
    let mut assemblies = Vec::new();

    unsafe {
        let raw_assemblies_ret =
            il2cpp_sys::il2cpp_domain_get_assemblies(domain, &mut size as *mut usize);

        match raw_assemblies_ret {
            Ok(raw_assemblies) => {
                if !raw_assemblies.is_null() && size > 0 {
                    for i in 0..size {
                        let assembly_ptr = *raw_assemblies.add(i);
                        assemblies.push(assembly_ptr);
                    }
                }
            }
            Err(e) => return Err(format!("Failed to get assemblies: {}", e)),
        }
    }
    Ok(assemblies)
}

pub fn assembly_get_image(assembly: Il2CppAssembly) -> Result<Il2CppImage, String> {
    il2cpp_sys::il2cpp_assembly_get_image(assembly)
}

pub fn image_get_filename(image: Il2CppMethodInfo) -> Result<String, String> {
    match il2cpp_sys::il2cpp_image_get_filename(image) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Image filename is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }

        Err(e) => Err(format!("Failed to get image filename: {}", e)),
    }
}

pub fn image_get_name(image: Il2CppImage) -> Result<String, String> {
    match il2cpp_sys::il2cpp_image_get_name(image) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Image name is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }

        Err(e) => Err(format!("Failed to get image name: {}", e)),
    }
}

pub fn image_get_class(image: Il2CppImage, index: usize) -> Result<Il2CppClass, String> {
    il2cpp_sys::il2cpp_image_get_class(image, index)
}

pub fn image_get_class_count(image: Il2CppImage) -> Result<usize, String> {
    match il2cpp_sys::il2cpp_image_get_class_count(image) {
        Ok(count) => Ok(count),
        Err(e) => Err(format!("Failed to get image class count: {}", e)),
    }
}

pub fn class_from_name(
    image: Il2CppImage,
    namespace: &str,
    name: &str,
) -> Result<Il2CppClass, String> {
    let c_namespace = CString::new(namespace);
    let c_name = CString::new(name);
    if c_namespace.is_err() || c_name.is_err() {
        return Err("Failed to create c strings".to_string());
    }

    if c_namespace.clone().ok().is_none() || c_name.clone().ok().is_none() {
        return Err("Failed to create c strings".to_string());
    }

    il2cpp_sys::il2cpp_class_from_name(
        image,
        c_namespace.ok().unwrap().as_ptr(),
        c_name.ok().unwrap().as_ptr(),
    )
}

pub fn class_get_fields(klass: Il2CppClass) -> Result<Vec<*mut u8>, String> {
    let mut iter: usize = 0;
    let mut fields = Vec::new();

    match il2cpp_sys::il2cpp_class_get_fields(klass, &mut iter as *mut usize) {
        Ok(field_ptr) => {
            if field_ptr.is_null() {
                return Err("Field pointer is null".to_string());
            }
            fields.push(field_ptr);
        }
        Err(e) => return Err(format!("Failed to get fields: {}", e)),
    }
    Ok(fields)
}

pub fn class_get_name(klass: Il2CppClass) -> Result<String, String> {
    match il2cpp_sys::il2cpp_class_get_name(klass) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Class name is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }

        Err(e) => Err(format!("Failed to get class name: {}", e)),
    }
}

pub fn class_get_namespace(klass: Il2CppClass) -> Result<String, String> {
    match il2cpp_sys::il2cpp_class_get_namespace(klass) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Class namespace is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }

        Err(e) => Err(format!("Failed to get class namespace: {}", e)),
    }
}

pub fn class_get_parent(klass: Il2CppClass) -> Result<Il2CppClass, String> {
    il2cpp_sys::il2cpp_class_get_parent(klass)
}

pub fn field_get_name(field: *mut u8) -> Result<String, String> {
    match il2cpp_sys::il2cpp_field_get_name(field) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Field name is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }
        Err(e) => Err(format!("Failed to get field name: {}", e)),
    }
}

pub fn field_get_offset(field: *mut u8) -> Result<u32, String> {
    il2cpp_sys::il2cpp_field_get_offset(field)
}

pub fn class_get_methods(klass: Il2CppClass) -> Result<Vec<Il2CppMethodInfo>, String> {
    let mut iter: usize = 0;
    let mut methods = Vec::new();

    match il2cpp_sys::il2cpp_class_get_methods(klass, &mut iter as *mut usize) {
        Ok(method_ptr) => {
            if method_ptr.is_null() {
                return Err("Method pointer is null".to_string());
            }
            methods.push(method_ptr);
            Ok(methods)
        }
        Err(e) => Err(format!("Failed to get methods: {}", e)),
    }
}

pub fn method_get_name(method: Il2CppMethodInfo) -> Result<String, String> {
    match il2cpp_sys::il2cpp_method_get_name(method) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Method name is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }
        Err(e) => Err(format!("Failed to get method name: {}", e)),
    }
}

pub fn method_get_param_count(method: Il2CppMethodInfo) -> Result<u32, String> {
    il2cpp_sys::il2cpp_method_get_param_count(method)
}

pub fn method_get_param_name(method: Il2CppMethodInfo, index: u32) -> Result<String, String> {
    match il2cpp_sys::il2cpp_method_get_param_name(method, index) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Method param name is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }
        Err(e) => Err(format!("Failed to get method param name: {}", e)),
    }
}

pub fn method_get_return_type(method: Il2CppMethodInfo) -> Result<Il2CppType, String> {
    il2cpp_sys::il2cpp_method_get_return_type(method)
}

pub fn type_get_name(itype: Il2CppType) -> Result<String, String> {
    match il2cpp_sys::il2cpp_type_get_name(itype) {
        Ok(c_str) => {
            if c_str.is_null() {
                Err("Type name is null".to_string())
            } else {
                Ok(unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() })
            }
        }
        Err(e) => Err(format!("Failed to get type name: {}", e)),
    }
}

pub fn print_all_function_ptrs() {
    il2cpp_sys::il2cpp_print_all_function_ptrs();
}

pub fn init(module_name: &str) -> Result<(), String> {
    il2cpp_sys::initialize_il2cpp(module_name)
}
