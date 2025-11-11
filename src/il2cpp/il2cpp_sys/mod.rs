use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use win32_sys::{get_module_from_name, resolve_function_ptr_from_name};
use windows::Win32::Foundation::{FARPROC, HMODULE};

pub mod c_types;
pub mod fn_types;
mod win32_sys;

use c_types::*;
use fn_types::*;

struct Il2CppFunctions {
    pub init: Option<Il2CppInitFn>,
    pub shutdown: Option<Il2CppShutdownFn>,
    pub get_domain: Option<Il2CppGetDomainFn>,
    pub thread_attach: Option<Il2CppThreadAttachFn>,
    pub thread_detach: Option<Il2CppThreadDetachFn>,
    pub assembly_get_image: Option<Il2CppAssemblyGetImageFn>,
    pub class_from_name: Option<Il2CppClassFromNameFn>,
    pub class_get_methods: Option<Il2CppClassGetMethodsFn>,
    pub class_get_name: Option<Il2CppClassGetNameFn>,
    pub class_get_namespace: Option<Il2CppClassGetNamespaceFn>,
    pub class_get_parent: Option<Il2CppClassGetParentFn>,
    pub class_get_type: Option<Il2CppClassGetTypeFn>,
    pub method_get_name: Option<Il2CppMethodGetNameFn>,
    pub domain_get_assemblies: Option<Il2CppDomainGetAssembliesFn>,
    pub image_get_name: Option<Il2CppImageGetNameFn>,
    pub image_get_filename: Option<Il2CppImageGetFileNameFn>,
    pub image_get_class: Option<Il2CppImageGetClassFn>,
    pub image_get_class_count: Option<Il2CppImageGetClassCountFn>,
    pub class_get_fields: Option<Il2CppClassGetFieldsFn>,
    pub field_get_name: Option<Il2CppFieldGetNameFn>,
    pub field_get_offset: Option<Il2CppFieldGetOffsetFn>,
    pub field_get_type: Option<Il2CppFieldGetTypeFn>,
    pub method_get_param_count: Option<Il2CppMethodGetParamCountFn>,
    pub method_get_param_name: Option<Il2CppMethodGetParamNameFn>,
    pub method_get_return_type: Option<Il2CppMethodGetReturnTypeFn>,
    pub method_get_flags: Option<Il2CppMethodGetFlagsFn>,
    pub method_get_param: Option<Il2CppMethodGetParamFn>,
    pub type_get_name: Option<Il2CppTypeGetNameFn>,
}

impl Il2CppFunctions {
    pub fn default() -> Self {
        Self {
            init: None,
            shutdown: None,
            get_domain: None,
            thread_attach: None,
            thread_detach: None,
            assembly_get_image: None,
            class_from_name: None,
            class_get_methods: None,
            class_get_name: None,
            class_get_namespace: None,
            class_get_parent: None,
            class_get_type: None,
            method_get_name: None,
            domain_get_assemblies: None,
            image_get_name: None,
            image_get_filename: None,
            image_get_class: None,
            image_get_class_count: None,
            class_get_fields: None,
            field_get_name: None,
            field_get_offset: None,
            field_get_type: None,
            method_get_param_count: None,
            method_get_param_name: None,
            method_get_return_type: None,
            method_get_flags: None,
            method_get_param: None,
            type_get_name: None,
        }
    }
}

struct Il2CppDll {
    name: String,
    module: HMODULE,
    // cache to speed up
    cache: HashMap<String, FARPROC>,
    // il2cpp functions
    functions: Il2CppFunctions,
}

impl Il2CppDll {
    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            module: HMODULE::default(),
            cache: HashMap::new(),
            functions: Il2CppFunctions::default(),
        }
    }
    #[allow(dead_code)]
    pub fn new(module_name: &str) -> Result<Self, String> {
        match get_module_from_name(module_name) {
            Ok(module) => Ok(Self {
                name: module_name.to_string(),
                module,
                cache: HashMap::new(),
                functions: Il2CppFunctions::default(),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn is_valid(&self) -> bool {
        return !self.module.is_invalid();
    }

    pub fn cached_resolve_function_ptr_from_name(
        &self,
        name: &str,
    ) -> Result<Option<FARPROC>, String> {
        if !self.is_valid() {
            return Err(format!("Module {} is not valid", self.name));
        }

        match self.cache.get(name) {
            Some(addr) => Ok(Some(*addr)),
            None => Ok(None), //Err(format!("Function {} not found in cache", name)),
        }
    }

    pub fn invoke<T>(&self, name: &str) -> Result<T, String> {
        if !self.is_valid() {
            return Err(format!("Module {} is not valid", self.name));
        }

        // try lookup in the cache
        match self.cached_resolve_function_ptr_from_name(name) {
            Ok(Some(proc_address)) => {
                return Ok(unsafe { std::mem::transmute_copy::<FARPROC, T>(&proc_address) });
            }
            Ok(None) => {} // continue and look it up instead
            Err(e) => return Err(e),
        }

        match resolve_function_ptr_from_name(self.module, name) {
            Ok(Some(proc_address)) => {
                Ok(unsafe { std::mem::transmute_copy::<FARPROC, T>(&proc_address) })
            }
            Ok(None) => Err(format!("Failed to resolve function {}", name)),
            Err(e) => return Err(e),
        }
    }

    // Invoking with a mutable reference allows your function to be cached
    pub fn invoke_mut<T>(&mut self, name: &str) -> Result<T, String> {
        if !self.is_valid() {
            return Err(format!("Module {} is not valid", self.name));
        }

        // try lookup in the cache
        match self.cached_resolve_function_ptr_from_name(name) {
            Ok(Some(proc_address)) => {
                return Ok(unsafe { std::mem::transmute_copy::<FARPROC, T>(&proc_address) });
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }

        match resolve_function_ptr_from_name(self.module, name) {
            Ok(Some(proc_address)) => {
                self.cache.insert(name.to_string(), proc_address);
                Ok(unsafe { std::mem::transmute_copy::<FARPROC, T>(&proc_address) })
            }
            Ok(None) => Err(format!("Failed to resolve function {}", name)),
            Err(e) => return Err(e),
        }
    }

    pub fn cache_function_addresses(&mut self) -> Result<(), String> {
        for name in IL2CPP_FUNCTIONS_NAMES.iter() {
            // this will cache all the functions
            match self.invoke_mut::<FARPROC>(name) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to cache function {}: {}", name, e)),
            }
        }

        Ok(())
    }

    pub fn cache_functions(&mut self) -> Result<(), String> {
        self.functions.init = Some(self.invoke_mut::<Il2CppInitFn>("il2cpp_init")?);
        self.functions.shutdown = Some(self.invoke_mut::<Il2CppShutdownFn>("il2cpp_shutdown")?);
        self.functions.get_domain =
            Some(self.invoke_mut::<Il2CppGetDomainFn>("il2cpp_domain_get")?);
        self.functions.thread_attach =
            Some(self.invoke_mut::<Il2CppThreadAttachFn>("il2cpp_thread_attach")?);
        self.functions.thread_detach =
            Some(self.invoke_mut::<Il2CppThreadDetachFn>("il2cpp_thread_detach")?);
        self.functions.assembly_get_image =
            Some(self.invoke_mut::<Il2CppAssemblyGetImageFn>("il2cpp_assembly_get_image")?);
        self.functions.class_from_name =
            Some(self.invoke_mut::<Il2CppClassFromNameFn>("il2cpp_class_from_name")?);
        self.functions.class_get_methods =
            Some(self.invoke_mut::<Il2CppClassGetMethodsFn>("il2cpp_class_get_methods")?);
        self.functions.class_get_name =
            Some(self.invoke_mut::<Il2CppClassGetNameFn>("il2cpp_class_get_name")?);
        self.functions.class_get_namespace =
            Some(self.invoke_mut::<Il2CppClassGetNamespaceFn>("il2cpp_class_get_namespace")?);
        self.functions.class_get_parent =
            Some(self.invoke_mut::<Il2CppClassGetParentFn>("il2cpp_class_get_parent")?);
        self.functions.class_get_type =
            Some(self.invoke_mut::<Il2CppClassGetTypeFn>("il2cpp_class_get_type")?);
        self.functions.method_get_name =
            Some(self.invoke_mut::<Il2CppMethodGetNameFn>("il2cpp_method_get_name")?);
        self.functions.domain_get_assemblies =
            Some(self.invoke_mut::<Il2CppDomainGetAssembliesFn>("il2cpp_domain_get_assemblies")?);
        self.functions.image_get_name =
            Some(self.invoke_mut::<Il2CppImageGetNameFn>("il2cpp_image_get_name")?);
        self.functions.image_get_filename =
            Some(self.invoke_mut::<Il2CppImageGetFileNameFn>("il2cpp_image_get_filename")?);
        self.functions.image_get_class =
            Some(self.invoke_mut::<Il2CppImageGetClassFn>("il2cpp_image_get_class")?);
        self.functions.image_get_class_count =
            Some(self.invoke_mut::<Il2CppImageGetClassCountFn>("il2cpp_image_get_class_count")?);
        self.functions.class_get_fields =
            Some(self.invoke_mut::<Il2CppClassGetFieldsFn>("il2cpp_class_get_fields")?);
        self.functions.field_get_name =
            Some(self.invoke_mut::<Il2CppFieldGetNameFn>("il2cpp_field_get_name")?);
        self.functions.field_get_offset =
            Some(self.invoke_mut::<Il2CppFieldGetOffsetFn>("il2cpp_field_get_offset")?);
        self.functions.field_get_type =
            Some(self.invoke_mut::<Il2CppFieldGetTypeFn>("il2cpp_field_get_type")?);
        self.functions.method_get_param_count =
            Some(self.invoke_mut::<Il2CppMethodGetParamCountFn>("il2cpp_method_get_param_count")?);
        self.functions.method_get_param_name =
            Some(self.invoke_mut::<Il2CppMethodGetParamNameFn>("il2cpp_method_get_param_name")?);
        self.functions.method_get_return_type =
            Some(self.invoke_mut::<Il2CppMethodGetReturnTypeFn>("il2cpp_method_get_return_type")?);
        self.functions.method_get_flags =
            Some(self.invoke_mut::<Il2CppMethodGetFlagsFn>("il2cpp_method_get_flags")?);
        self.functions.method_get_param =
            Some(self.invoke_mut::<Il2CppMethodGetParamFn>("il2cpp_method_get_param")?);
        self.functions.type_get_name =
            Some(self.invoke_mut::<Il2CppTypeGetNameFn>("il2cpp_type_get_name")?);
        Ok(())
    }

    pub fn print_all_functions(&self) {
        println!("Il2Cpp functions:");
        println!("il2cpp_init: {:?}", self.functions.init);
        println!("il2cpp_shutdown: {:?}", self.functions.shutdown);
        println!("il2cpp_domain_get: {:?}", self.functions.get_domain);
        println!("il2cpp_thread_attach: {:?}", self.functions.thread_attach);
        println!("il2cpp_thread_detach: {:?}", self.functions.thread_detach);
        println!(
            "il2cpp_assembly_get_image: {:?}",
            self.functions.assembly_get_image
        );
        println!(
            "il2cpp_class_from_name: {:?}",
            self.functions.class_from_name
        );
        println!("il2cpp_class_get_type: {:?}", self.functions.class_get_type);
        println!(
            "il2cpp_class_get_methods: {:?}",
            self.functions.class_get_methods
        );
        println!(
            "il2cpp_method_get_name: {:?}",
            self.functions.method_get_name
        );
        println!(
            "il2cpp_domain_get_assemblies: {:?}",
            self.functions.domain_get_assemblies
        );
        println!("il2cpp_image_get_name: {:?}", self.functions.image_get_name);
        println!(
            "il2cpp_image_get_filename: {:?}",
            self.functions.image_get_filename
        );
        println!(
            "il2cpp_class_get_fields: {:?}",
            self.functions.class_get_fields
        );
        println!("il2cpp_field_get_name: {:?}", self.functions.field_get_name);
        println!(
            "il2cpp_field_get_offset: {:?}",
            self.functions.field_get_offset
        );
        println!("il2cpp_field_get_type: {:?}", self.functions.field_get_type);
        println!(
            "il2cpp_method_get_param_count: {:?}",
            self.functions.method_get_param_count
        );
        println!(
            "il2cpp_method_get_param_name: {:?}",
            self.functions.method_get_param_name
        );
        println!(
            "il2cpp_method_get_return_type: {:?}",
            self.functions.method_get_return_type
        );
        println!(
            "il2cpp_method_get_flags: {:?}",
            self.functions.method_get_flags
        );
        println!(
            "il2cpp_method_get_param: {:?}",
            self.functions.method_get_param
        );
        println!("il2cpp_type_get_name: {:?}", self.functions.type_get_name);
    }

    pub fn il2cpp_init(&self) -> Result<(), String> {
        match self.functions.init {
            Some(init) => Ok(unsafe { init() }),
            None => match self.invoke::<Il2CppInitFn>("il2cpp_init") {
                Ok(init) => Ok(unsafe { init() }),
                Err(e) => Err(format!("Failed to invoke il2cpp_init: {}", e)),
            },
        }
    }

    pub fn il2cpp_shutdown(&self) -> Result<(), String> {
        match self.functions.shutdown {
            Some(shutdown) => Ok(unsafe { shutdown() }),
            None => match self.invoke::<Il2CppShutdownFn>("il2cpp_shutdown") {
                Ok(shutdown) => Ok(unsafe { shutdown() }),
                Err(e) => Err(format!("Failed to invoke il2cpp_shutdown: {}", e)),
            },
        }
    }

    pub fn il2cpp_domain_get(&self) -> Result<Il2CppDomain, String> {
        match self.functions.get_domain {
            Some(get_domain) => Ok(unsafe { get_domain() }),
            None => match self.invoke::<Il2CppGetDomainFn>("il2cpp_domain_get") {
                Ok(get_domain) => Ok(unsafe { get_domain() }),
                Err(e) => Err(format!("Failed to invoke il2cpp_domain_get: {}", e)),
            },
        }
    }

    pub fn il2cpp_thread_attach(&self, domain: Il2CppDomain) -> Result<Il2CppThread, String> {
        match self.functions.thread_attach {
            Some(thread_attach) => Ok(unsafe { thread_attach(domain) }),
            None => match self.invoke::<Il2CppThreadAttachFn>("il2cpp_thread_attach") {
                Ok(thread_attach) => Ok(unsafe { thread_attach(domain) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_thread_attach: {}", e)),
            },
        }
    }

    pub fn il2cpp_thread_detach(&self, thread: Il2CppThread) -> Result<(), String> {
        match self.functions.thread_detach {
            Some(thread_detach) => Ok(unsafe { thread_detach(thread) }),
            None => match self.invoke::<Il2CppThreadDetachFn>("il2cpp_thread_detach") {
                Ok(thread_detach) => Ok(unsafe { thread_detach(thread) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_thread_detach: {}", e)),
            },
        }
    }

    pub fn il2cpp_assembly_get_image(
        &self,
        assembly: Il2CppAssembly,
    ) -> Result<Il2CppImage, String> {
        match self.functions.assembly_get_image {
            Some(assembly_get_image) => Ok(unsafe { assembly_get_image(assembly) }),
            None => match self.invoke::<Il2CppAssemblyGetImageFn>("il2cpp_assembly_get_image") {
                Ok(assembly_get_image) => Ok(unsafe { assembly_get_image(assembly) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_assembly_get_image: {}", e)),
            },
        }
    }

    pub fn il2cpp_class_from_name(
        &self,
        image: Il2CppImage,
        namespace: *const i8,
        name: *const i8,
    ) -> Result<Il2CppClass, String> {
        match self.functions.class_from_name {
            Some(class_from_name) => Ok(unsafe { class_from_name(image, namespace, name) }),
            None => match self.invoke::<Il2CppClassFromNameFn>("il2cpp_class_from_name") {
                Ok(class_from_name) => Ok(unsafe { class_from_name(image, namespace, name) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_class_from_name: {}", e)),
            },
        }
    }

    pub fn il2cpp_class_get_methods(
        &self,
        klass: Il2CppClass,
        iter: *mut *mut u8,
    ) -> Result<Il2CppMethodInfo, String> {
        match self.functions.class_get_methods {
            Some(class_get_methods) => Ok(unsafe { class_get_methods(klass, iter) }),
            None => match self.invoke::<Il2CppClassGetMethodsFn>("il2cpp_class_get_methods") {
                Ok(class_get_methods) => Ok(unsafe { class_get_methods(klass, iter) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_class_get_methods: {}", e)),
            },
        }
    }

    pub fn il2cpp_method_get_name(&self, method: Il2CppMethodInfo) -> Result<*const i8, String> {
        match self.functions.method_get_name {
            Some(method_get_name) => Ok(unsafe { method_get_name(method) }),
            None => match self.invoke::<Il2CppMethodGetNameFn>("il2cpp_method_get_name") {
                Ok(method_get_name) => Ok(unsafe { method_get_name(method) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_method_get_name: {}", e)),
            },
        }
    }

    pub fn il2cpp_domain_get_assemblies(
        &self,
        domain: Il2CppDomain,
        size: *mut usize,
    ) -> Result<*mut Il2CppAssembly, String> {
        match self.functions.domain_get_assemblies {
            Some(domain_get_assemblies) => Ok(unsafe { domain_get_assemblies(domain, size) }),
            None => {
                match self.invoke::<Il2CppDomainGetAssembliesFn>("il2cpp_domain_get_assemblies") {
                    Ok(domain_get_assemblies) => Ok(unsafe { domain_get_assemblies(domain, size) }),
                    Err(e) => Err(format!(
                        "Failed to invoke il2cpp_domain_get_assemblies: {}",
                        e
                    )),
                }
            }
        }
    }

    pub fn il2cpp_image_get_name(&self, image: Il2CppImage) -> Result<*const i8, String> {
        match self.functions.image_get_name {
            Some(image_get_name) => Ok(unsafe { image_get_name(image) }),
            None => match self.invoke::<Il2CppImageGetNameFn>("il2cpp_image_get_name") {
                Ok(image_get_name) => Ok(unsafe { image_get_name(image) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_image_get_name: {}", e)),
            },
        }
    }

    pub fn il2cpp_image_get_filename(&self, image: Il2CppImage) -> Result<*const i8, String> {
        match self.functions.image_get_filename {
            Some(image_get_filename) => Ok(unsafe { image_get_filename(image) }),
            None => match self.invoke::<Il2CppImageGetFileNameFn>("il2cpp_image_get_filename") {
                Ok(image_get_filename) => Ok(unsafe { image_get_filename(image) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_image_get_filename: {}", e)),
            },
        }
    }

    pub fn il2cpp_class_get_fields(
        &self,
        klass: Il2CppClass,
        iter: *mut *mut u8,
    ) -> Result<*mut u8, String> {
        match self.functions.class_get_fields {
            Some(class_get_fields) => Ok(unsafe { class_get_fields(klass, iter) }),
            None => match self.invoke::<Il2CppClassGetFieldsFn>("il2cpp_class_get_fields") {
                Ok(class_get_fields) => Ok(unsafe { class_get_fields(klass, iter) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_class_get_fields: {}", e)),
            },
        }
    }

    pub fn il2cpp_field_get_name(&self, field: Il2CppFieldInfo) -> Result<*const i8, String> {
        match self.functions.field_get_name {
            Some(field_get_name) => Ok(unsafe { field_get_name(field) }),
            None => match self.invoke::<Il2CppFieldGetNameFn>("il2cpp_field_get_name") {
                Ok(field_get_name) => Ok(unsafe { field_get_name(field) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_field_get_name: {}", e)),
            },
        }
    }

    pub fn il2cpp_field_get_offset(&self, field: Il2CppFieldInfo) -> Result<i32, String> {
        match self.functions.field_get_offset {
            Some(field_get_offset) => Ok(unsafe { field_get_offset(field) }),
            None => match self.invoke::<Il2CppFieldGetOffsetFn>("il2cpp_field_get_offset") {
                Ok(field_get_offset) => Ok(unsafe { field_get_offset(field) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_field_get_offset: {}", e)),
            },
        }
    }

    pub fn il2cpp_method_get_param_count(&self, method: Il2CppMethodInfo) -> Result<u32, String> {
        match self.functions.method_get_param_count {
            Some(method_get_param_count) => Ok(unsafe { method_get_param_count(method) }),
            None => {
                match self.invoke::<Il2CppMethodGetParamCountFn>("il2cpp_method_get_param_count") {
                    Ok(method_get_param_count) => Ok(unsafe { method_get_param_count(method) }),
                    Err(e) => Err(format!(
                        "Failed to invoke il2cpp_method_get_param_count: {}",
                        e
                    )),
                }
            }
        }
    }

    pub fn il2cpp_method_get_param_name(
        &self,
        method: Il2CppMethodInfo,
        index: u32,
    ) -> Result<*const i8, String> {
        match self.functions.method_get_param_name {
            Some(method_get_param_name) => Ok(unsafe { method_get_param_name(method, index) }),
            None => match self.invoke::<Il2CppMethodGetParamNameFn>("il2cpp_method_get_param_name")
            {
                Ok(method_get_param_name) => Ok(unsafe { method_get_param_name(method, index) }),
                Err(e) => Err(format!(
                    "Failed to invoke il2cpp_method_get_param_name: {}",
                    e
                )),
            },
        }
    }

    pub fn il2cpp_method_get_return_type(
        &self,
        method: Il2CppMethodInfo,
    ) -> Result<Il2CppType, String> {
        match self.functions.method_get_return_type {
            Some(method_get_return_type) => Ok(unsafe { method_get_return_type(method) }),
            None => {
                match self.invoke::<Il2CppMethodGetReturnTypeFn>("il2cpp_method_get_return_type") {
                    Ok(method_get_return_type) => Ok(unsafe { method_get_return_type(method) }),
                    Err(e) => Err(format!(
                        "Failed to invoke il2cpp_method_get_return_type: {}",
                        e
                    )),
                }
            }
        }
    }

    pub fn il2cpp_method_get_flags(
        &self,
        method: Il2CppMethodInfo,
        iflag: *mut i32,
    ) -> Result<i32, String> {
        match self.functions.method_get_flags {
            Some(method_get_flags) => Ok(unsafe { method_get_flags(method, iflag) }),
            None => match self.invoke::<Il2CppMethodGetFlagsFn>("il2cpp_method_get_flags") {
                Ok(method_get_flags) => Ok(unsafe { method_get_flags(method, iflag) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_method_get_flags: {}", e)),
            },
        }
    }

    pub fn il2cpp_method_get_param(
        &self,
        method: Il2CppMethodInfo,
        index: u32,
    ) -> Result<Il2CppType, String> {
        match self.functions.method_get_param {
            Some(method_get_param) => Ok(unsafe { method_get_param(method, index) }),
            None => match self.invoke::<Il2CppMethodGetParamFn>("il2cpp_method_get_param") {
                Ok(method_get_param) => Ok(unsafe { method_get_param(method, index) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_method_get_param: {}", e)),
            },
        }
    }

    pub fn il2cpp_type_get_name(&self, itype: Il2CppType) -> Result<*const i8, String> {
        match self.functions.type_get_name {
            Some(type_get_name) => Ok(unsafe { type_get_name(itype) }),
            None => match self.invoke::<Il2CppTypeGetNameFn>("il2cpp_type_get_name") {
                Ok(type_get_name) => Ok(unsafe { type_get_name(itype) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_type_get_name: {}", e)),
            },
        }
    }

    pub fn il2cpp_image_get_class(
        &self,
        image: Il2CppImage,
        index: usize,
    ) -> Result<Il2CppClass, String> {
        match self.functions.image_get_class {
            Some(image_get_class) => Ok(unsafe { image_get_class(image, index) }),
            None => match self.invoke::<Il2CppImageGetClassFn>("il2cpp_image_get_class") {
                Ok(image_get_class) => Ok(unsafe { image_get_class(image, index) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_image_get_class: {}", e)),
            },
        }
    }

    pub fn il2cpp_image_get_class_count(&self, image: Il2CppImage) -> Result<usize, String> {
        match self.functions.image_get_class_count {
            Some(image_get_class_count) => Ok(unsafe { image_get_class_count(image) }),
            None => match self.invoke::<Il2CppImageGetClassCountFn>("il2cpp_image_get_class_count")
            {
                Ok(image_get_class_count) => Ok(unsafe { image_get_class_count(image) }),
                Err(e) => Err(format!(
                    "Failed to invoke il2cpp_image_get_class_count: {}",
                    e
                )),
            },
        }
    }

    pub fn il2cpp_class_get_name(&self, klass: Il2CppClass) -> Result<*const i8, String> {
        match self.functions.class_get_name {
            Some(class_get_name) => Ok(unsafe { class_get_name(klass) }),
            None => match self.invoke::<Il2CppClassGetNameFn>("il2cpp_class_get_name") {
                Ok(class_get_name) => Ok(unsafe { class_get_name(klass) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_class_get_name: {}", e)),
            },
        }
    }

    pub fn il2cpp_class_get_namespace(&self, klass: Il2CppClass) -> Result<*const i8, String> {
        match self.functions.class_get_namespace {
            Some(class_get_namespace) => Ok(unsafe { class_get_namespace(klass) }),
            None => match self.invoke::<Il2CppClassGetNamespaceFn>("il2cpp_class_get_namespace") {
                Ok(class_get_namespace) => Ok(unsafe { class_get_namespace(klass) }),
                Err(e) => Err(format!(
                    "Failed to invoke il2cpp_class_get_namespace: {}",
                    e
                )),
            },
        }
    }

    pub fn il2cpp_class_get_parent(&self, klass: Il2CppClass) -> Result<Il2CppClass, String> {
        match self.functions.class_get_parent {
            Some(class_get_parent) => Ok(unsafe { class_get_parent(klass) }),
            None => match self.invoke::<Il2CppClassGetParentFn>("il2cpp_class_get_parent") {
                Ok(class_get_parent) => Ok(unsafe { class_get_parent(klass) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_class_get_parent: {}", e)),
            },
        }
    }

    pub fn il2cpp_field_get_type(&self, field: Il2CppFieldInfo) -> Result<Il2CppType, String> {
        match self.functions.field_get_type {
            Some(field_get_type) => Ok(unsafe { field_get_type(field) }),
            None => match self.invoke::<Il2CppFieldGetTypeFn>("il2cpp_field_get_type") {
                Ok(field_get_type) => Ok(unsafe { field_get_type(field) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_field_get_type: {}", e)),
            },
        }
    }

    pub fn il2cpp_class_get_type(&self, klass: Il2CppClass) -> Result<Il2CppType, String> {
        match self.functions.class_get_type {
            Some(class_get_type) => Ok(unsafe { class_get_type(klass) }),
            None => match self.invoke::<Il2CppClassGetTypeFn>("il2cpp_class_get_type") {
                Ok(class_get_type) => Ok(unsafe { class_get_type(klass) }),
                Err(e) => Err(format!("Failed to invoke il2cpp_class_get_type: {}", e)),
            },
        }
    }
}

unsafe impl Send for Il2CppDll {}
unsafe impl Sync for Il2CppDll {}

pub fn initialize_il2cpp(module_name: &str) -> Result<(), String> {
    match get_module_from_name(module_name) {
        Ok(module) => {
            let mut dll = IL2CPP_MODULE.write();
            dll.name = module_name.to_string();
            dll.module = module;

            // lookup il2cpp funcs
            match dll.cache_function_addresses() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            match dll.cache_functions() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

// public functions
pub fn il2cpp_init() -> Result<(), String> {
    IL2CPP_MODULE.read().il2cpp_init()
}

pub fn il2cpp_shutdown() -> Result<(), String> {
    IL2CPP_MODULE.read().il2cpp_shutdown()
}

pub fn il2cpp_domain_get() -> Result<Il2CppDomain, String> {
    IL2CPP_MODULE.read().il2cpp_domain_get()
}

pub fn il2cpp_thread_attach(domain: Il2CppDomain) -> Result<Il2CppThread, String> {
    IL2CPP_MODULE.read().il2cpp_thread_attach(domain)
}

pub fn il2cpp_thread_detach(domain: Il2CppDomain) -> Result<(), String> {
    IL2CPP_MODULE.read().il2cpp_thread_detach(domain)
}

pub fn il2cpp_assembly_get_image(assembly: Il2CppAssembly) -> Result<Il2CppImage, String> {
    IL2CPP_MODULE.read().il2cpp_assembly_get_image(assembly)
}

pub fn il2cpp_class_from_name(
    image: Il2CppImage,
    namespace: *const i8,
    name: *const i8,
) -> Result<Il2CppClass, String> {
    IL2CPP_MODULE
        .read()
        .il2cpp_class_from_name(image, namespace, name)
}

pub fn il2cpp_class_get_methods(
    klass: Il2CppClass,
    iter: *mut *mut u8,
) -> Result<Il2CppMethodInfo, String> {
    IL2CPP_MODULE.read().il2cpp_class_get_methods(klass, iter)
}

pub fn il2cpp_class_get_name(klass: Il2CppClass) -> Result<*const i8, String> {
    IL2CPP_MODULE.read().il2cpp_class_get_name(klass)
}

pub fn il2cpp_class_get_namespace(klass: Il2CppClass) -> Result<*const i8, String> {
    IL2CPP_MODULE.read().il2cpp_class_get_namespace(klass)
}

pub fn il2cpp_class_get_parent(klass: Il2CppClass) -> Result<Il2CppClass, String> {
    IL2CPP_MODULE.read().il2cpp_class_get_parent(klass)
}

pub fn il2cpp_method_get_name(method: Il2CppMethodInfo) -> Result<*const i8, String> {
    IL2CPP_MODULE.read().il2cpp_method_get_name(method)
}

pub fn il2cpp_domain_get_assemblies(
    domain: Il2CppDomain,
    size: *mut usize,
) -> Result<*mut Il2CppAssembly, String> {
    IL2CPP_MODULE
        .read()
        .il2cpp_domain_get_assemblies(domain, size)
}

pub fn il2cpp_image_get_name(image: Il2CppImage) -> Result<*const i8, String> {
    IL2CPP_MODULE.read().il2cpp_image_get_name(image)
}

pub fn il2cpp_image_get_filename(image: Il2CppImage) -> Result<*const i8, String> {
    IL2CPP_MODULE.read().il2cpp_image_get_filename(image)
}

pub fn il2cpp_image_get_class(image: Il2CppImage, index: usize) -> Result<Il2CppClass, String> {
    IL2CPP_MODULE.read().il2cpp_image_get_class(image, index)
}

pub fn il2cpp_image_get_class_count(image: Il2CppImage) -> Result<usize, String> {
    IL2CPP_MODULE.read().il2cpp_image_get_class_count(image)
}

pub fn il2cpp_class_get_fields(klass: Il2CppClass, iter: *mut *mut u8) -> Result<*mut u8, String> {
    IL2CPP_MODULE.read().il2cpp_class_get_fields(klass, iter)
}

pub fn il2cpp_field_get_name(field: Il2CppFieldInfo) -> Result<*const i8, String> {
    IL2CPP_MODULE.read().il2cpp_field_get_name(field)
}

pub fn il2cpp_field_get_offset(field: Il2CppFieldInfo) -> Result<i32, String> {
    IL2CPP_MODULE.read().il2cpp_field_get_offset(field)
}

pub fn il2cpp_field_get_type(field: Il2CppFieldInfo) -> Result<Il2CppType, String> {
    IL2CPP_MODULE.read().il2cpp_field_get_type(field)
}

pub fn il2cpp_method_get_param_count(method: Il2CppMethodInfo) -> Result<u32, String> {
    IL2CPP_MODULE.read().il2cpp_method_get_param_count(method)
}

pub fn il2cpp_method_get_param_name(
    method: Il2CppMethodInfo,
    index: u32,
) -> Result<*const i8, String> {
    IL2CPP_MODULE
        .read()
        .il2cpp_method_get_param_name(method, index)
}

pub fn il2cpp_method_get_return_type(method: Il2CppMethodInfo) -> Result<Il2CppType, String> {
    IL2CPP_MODULE.read().il2cpp_method_get_return_type(method)
}

pub fn il2cpp_method_get_flags(method: Il2CppMethodInfo, iflag: *mut i32) -> Result<i32, String> {
    IL2CPP_MODULE.read().il2cpp_method_get_flags(method, iflag)
}

pub fn il2cpp_method_get_param(method: Il2CppMethodInfo, index: u32) -> Result<Il2CppType, String> {
    IL2CPP_MODULE.read().il2cpp_method_get_param(method, index)
}

pub fn il2cpp_type_get_name(itype: Il2CppType) -> Result<*const i8, String> {
    IL2CPP_MODULE.read().il2cpp_type_get_name(itype)
}

pub fn il2cpp_class_get_type(klass: Il2CppClass) -> Result<Il2CppType, String> {
    IL2CPP_MODULE.read().il2cpp_class_get_type(klass)
}

pub fn il2cpp_print_all_function_ptrs() {
    IL2CPP_MODULE.read().print_all_functions();
}

static IL2CPP_MODULE: LazyLock<Arc<RwLock<Il2CppDll>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Il2CppDll::default())));

static IL2CPP_FUNCTIONS_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    Vec::from([
        "il2cpp_init".to_string(),
        "il2cpp_shutdown".to_string(),
        "il2cpp_domain_get".to_string(),
        "il2cpp_thread_attach".to_string(),
        "il2cpp_thread_detach".to_string(),
        "il2cpp_assembly_get_image".to_string(),
        "il2cpp_class_from_name".to_string(),
        "il2cpp_class_get_methods".to_string(),
        "il2cpp_method_get_name".to_string(),
        "il2cpp_domain_get_assemblies".to_string(),
        "il2cpp_image_get_name".to_string(),
        "il2cpp_image_get_filename".to_string(),
        "il2cpp_class_get_fields".to_string(),
        "il2cpp_field_get_name".to_string(),
        "il2cpp_field_get_offset".to_string(),
        "il2cpp_field_get_type".to_string(),
        "il2cpp_method_get_param_count".to_string(),
        "il2cpp_method_get_param_name".to_string(),
        "il2cpp_method_get_return_type".to_string(),
        "il2cpp_method_get_flags".to_string(),
        "il2cpp_class_get_type".to_string(),
    ])
});
