use std::ffi::CString;
use windows::Win32::Foundation::{FARPROC, HMODULE};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::core::PCSTR;

// Basic types used by IL2CPP
pub type Il2CppDomain = *mut u8;
pub type Il2CppAssembly = *mut u8;
pub type Il2CppImage = *mut u8;
pub type Il2CppClass = *mut u8;
pub type Il2CppMethodInfo = *mut u8;
pub type Il2CppType = *mut u8;
pub type Il2CppObject = *mut u8;
pub type Il2CppString = *mut u8;
pub type Il2CppArray = *mut u8;
pub type Il2CppManagedMemoryCallbacks = *mut u8;
pub type Il2CppManagedMemorySnapshot = *mut u8;
pub type Il2CppSetFindAttributeFunc = *mut u8;
pub type Il2CppSetIterateAttributeFunc = *mut u8;
pub type Il2CppSetClassArrayAllocatorFunc = *mut u8;
pub type Il2CppSetGenericArrayAllocatorFunc = *mut u8;
pub type Il2CppThread = *mut u8;
pub type Il2CppSyncDelegate = *mut u8;
pub type Il22CppCodeGenModule = *mut u8;
pub type Il2CppMemorySnapshot = *mut u8;
pub type Il2CppMetadataRegistration = *mut u8;
pub type Il2CppCodeRegistration = *mut u8;

// Function pointer types for IL2CPP API
type Il2CppInitFn = unsafe extern "C" fn();
type Il2CppShutdownFn = unsafe extern "C" fn();
type Il2CppGetDomainFn = unsafe extern "C" fn() -> Il2CppDomain;
type Il2CppThreadAttachFn = unsafe extern "C" fn(domain: Il2CppDomain);
type Il2CppThreadDetachFn = unsafe extern "C" fn(domain: Il2CppDomain);
type Il2CppAssemblyGetImageFn = unsafe extern "C" fn(assembly: Il2CppAssembly) -> Il2CppImage;
type Il2CppClassFromNameFn =
    unsafe extern "C" fn(image: Il2CppImage, namespace: *const i8, name: *const i8) -> Il2CppClass;
type Il2CppClassGetMethodsFn =
    unsafe extern "C" fn(klass: Il2CppClass, iter: *mut usize) -> Il2CppMethodInfo;
type Il2CppMethodGetNameFn = unsafe extern "C" fn(method: Il2CppMethodInfo) -> *const i8;
type Il2CppDomainGetAssembliesFn =
    unsafe extern "C" fn(domain: Il2CppDomain, size: *mut usize) -> *mut Il2CppAssembly;
type Il2CppImageGetNameFn = unsafe extern "C" fn(image: Il2CppImage) -> *const i8;
type Il2CppClassGetFieldsFn = unsafe extern "C" fn(klass: Il2CppClass, iter: *mut usize) -> *mut u8; // Il2CppField
type Il2CppFieldGetNameFn = unsafe extern "C" fn(field: *mut u8) -> *const i8;
type Il2CppFieldGetOffsetFn = unsafe extern "C" fn(field: *mut u8) -> u32;
type Il2CppMethodGetParamCountFn = unsafe extern "C" fn(method: Il2CppMethodInfo) -> u32;
type Il2CppMethodGetParamNameFn =
    unsafe extern "C" fn(method: Il2CppMethodInfo, index: u32) -> *const i8;
type Il2CppMethodGetReturnTypeFn = unsafe extern "C" fn(method: Il2CppMethodInfo) -> Il2CppType;
type Il2CppTypeGetNameFn = unsafe extern "C" fn(itype: Il2CppType) -> *const i8;

// A struct to hold all resolved IL2CPP function pointers
pub struct Il2CppApi {
    pub init: Option<Il2CppInitFn>,
    pub shutdown: Option<Il2CppShutdownFn>,
    pub get_domain: Option<Il2CppGetDomainFn>,
    pub thread_attach: Option<Il2CppThreadAttachFn>,
    pub thread_detach: Option<Il2CppThreadDetachFn>,
    pub assembly_get_image: Option<Il2CppAssemblyGetImageFn>,
    pub class_from_name: Option<Il2CppClassFromNameFn>,
    pub class_get_methods: Option<Il2CppClassGetMethodsFn>,
    pub method_get_name: Option<Il2CppMethodGetNameFn>,
    pub domain_get_assemblies: Option<Il2CppDomainGetAssembliesFn>,
    pub image_get_name: Option<Il2CppImageGetNameFn>,
    pub class_get_fields: Option<Il2CppClassGetFieldsFn>,
    pub field_get_name: Option<Il2CppFieldGetNameFn>,
    pub field_get_offset: Option<Il2CppFieldGetOffsetFn>,
    pub method_get_param_count: Option<Il2CppMethodGetParamCountFn>,
    pub method_get_param_name: Option<Il2CppMethodGetParamNameFn>,
    pub method_get_return_type: Option<Il2CppMethodGetReturnTypeFn>,
    pub type_get_name: Option<Il2CppTypeGetNameFn>,
}

impl Il2CppApi {
    pub fn new() -> Self {
        Self {
            init: None,
            shutdown: None,
            get_domain: None,
            thread_attach: None,
            thread_detach: None,
            assembly_get_image: None,
            class_from_name: None,
            class_get_methods: None,
            method_get_name: None,
            domain_get_assemblies: None,
            image_get_name: None,
            class_get_fields: None,
            field_get_name: None,
            field_get_offset: None,
            method_get_param_count: None,
            method_get_param_name: None,
            method_get_return_type: None,
            type_get_name: None,
        }
    }

    /// Resolves a function pointer from a given module.
    /// Returns `Some(FARPROC)` if successful, `None` otherwise.
    unsafe fn resolve_function(module: HMODULE, name: &str) -> Option<FARPROC> {
        let c_name = CString::new(name).ok()?;
        // Explicitly convert the *const i8 to PCSTR
        let proc_address = unsafe {
            let addr = GetProcAddress(module, PCSTR::from_raw(c_name.as_ptr() as *const u8));
            //println!("GET PROC ADDRESS FOR {} return {:?}", name, addr);
            addr
        };
        if proc_address.is_some() {
            Some(proc_address)
        } else {
            None
        }
    }

    /// Initializes the `Il2CppApi` by resolving all function pointers from the specified module.
    /// Returns `Ok(())` if all essential functions are resolved, `Err` otherwise.
    pub unsafe fn initialize(&mut self, module_name: &str) -> Result<(), String> {
        let c_module_name = CString::new(module_name)
            .map_err(|e| format!("Failed to create CString for module name: {}", e))?;

        let module =
            unsafe { GetModuleHandleA(PCSTR::from_raw(c_module_name.as_ptr() as *const u8)) }
                .map_err(|e| format!("Failed to load library {}: {}", module_name, e))?;
        if module.is_invalid() {
            return Err(format!("Failed to load library: {}", module_name));
        }

        self.init = unsafe {
            Self::resolve_function(module, "il2cpp_init").map(|f| std::mem::transmute(f))
        };
        self.shutdown = unsafe {
            Self::resolve_function(module, "il2cpp_shutdown").map(|f| std::mem::transmute(f))
        };
        self.get_domain = unsafe {
            Self::resolve_function(module, "il2cpp_domain_get").map(|f| std::mem::transmute(f))
        };
        self.thread_attach = unsafe {
            Self::resolve_function(module, "il2cpp_thread_attach").map(|f| std::mem::transmute(f))
        };
        self.thread_detach = unsafe {
            Self::resolve_function(module, "il2cpp_thread_detach").map(|f| std::mem::transmute(f))
        };
        self.assembly_get_image = unsafe {
            Self::resolve_function(module, "il2cpp_assembly_get_image")
                .map(|f| std::mem::transmute(f))
        };
        self.class_from_name = unsafe {
            Self::resolve_function(module, "il2cpp_class_from_name").map(|f| std::mem::transmute(f))
        };
        self.class_get_methods = unsafe {
            Self::resolve_function(module, "il2cpp_class_get_methods")
                .map(|f| std::mem::transmute(f))
        };
        self.method_get_name = unsafe {
            Self::resolve_function(module, "il2cpp_method_get_name").map(|f| std::mem::transmute(f))
        };
        self.domain_get_assemblies = unsafe {
            Self::resolve_function(module, "il2cpp_domain_get_assemblies")
                .map(|f| std::mem::transmute(f))
        };
        self.image_get_name = unsafe {
            Self::resolve_function(module, "il2cpp_image_get_name").map(|f| std::mem::transmute(f))
        };
        self.class_get_fields = unsafe {
            Self::resolve_function(module, "il2cpp_class_get_fields")
                .map(|f| std::mem::transmute(f))
        };
        self.field_get_name = unsafe {
            Self::resolve_function(module, "il2cpp_field_get_name").map(|f| std::mem::transmute(f))
        };
        self.field_get_offset = unsafe {
            Self::resolve_function(module, "il2cpp_field_get_offset")
                .map(|f| std::mem::transmute(f))
        };
        self.method_get_param_count = unsafe {
            Self::resolve_function(module, "il2cpp_method_get_param_count")
                .map(|f| std::mem::transmute(f))
        };
        self.method_get_param_name = unsafe {
            Self::resolve_function(module, "il2cpp_method_get_param_name")
                .map(|f| std::mem::transmute(f))
        };
        self.method_get_return_type = unsafe {
            Self::resolve_function(module, "il2cpp_method_get_return_type")
                .map(|f| std::mem::transmute(f))
        };
        self.type_get_name = unsafe {
            Self::resolve_function(module, "il2cpp_type_get_name").map(|f| std::mem::transmute(f))
        };

        // Basic check to ensure essential functions are resolved
        // Expanded to include a few more functions for a more robust check
        if self.init.is_none()
            || self.shutdown.is_none()
            || self.get_domain.is_none()
            || self.domain_get_assemblies.is_none()
            || self.assembly_get_image.is_none()
            || self.image_get_name.is_none()
            || self.class_from_name.is_none()
        {
            Err("Failed to resolve essential IL2CPP functions.".to_string())
        } else {
            Ok(())
        }
    }

    pub fn print_all_function_ptrs(&self) {
        println!("init: {:?}", self.init);
        println!("shutdown: {:?}", self.shutdown);
        println!("get_domain: {:?}", self.get_domain);
        println!("thread_attach: {:?}", self.thread_attach);
        println!("thread_detach: {:?}", self.thread_detach);
        println!("assembly_get_image: {:?}", self.assembly_get_image);
        println!("class_from_name: {:?}", self.class_from_name);
        println!("class_get_methods: {:?}", self.class_get_methods);
        println!("method_get_name: {:?}", self.method_get_name);
        println!("domain_get_assemblies: {:?}", self.domain_get_assemblies);
        println!("image_get_name: {:?}", self.image_get_name);
        println!("class_get_fields: {:?}", self.class_get_fields);
        println!("field_get_name: {:?}", self.field_get_name);
        println!("field_get_offset: {:?}", self.field_get_offset);
        println!("method_get_param_count: {:?}", self.method_get_param_count);
        println!("method_get_param_name: {:?}", self.method_get_param_name);
        println!("method_get_return_type: {:?}", self.method_get_return_type);
        println!("type_get_name: {:?}", self.type_get_name);
    }
}
