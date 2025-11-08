use std::ffi::CString;
use windows::Win32::Foundation::{FARPROC, HMODULE};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::core::PCSTR;

// Util to get module from name
pub fn get_module_from_name(module_name: &str) -> Result<HMODULE, String> {
    let c_module_name = CString::new(module_name)
        .map_err(|e| format!("Failed to create CString for module name: {}", e))?;

    let module = unsafe { GetModuleHandleA(PCSTR::from_raw(c_module_name.as_ptr() as *const u8)) }
        .map_err(|e| format!("Failed to load library {}: {}", module_name, e))?;
    if module.is_invalid() {
        return Err(format!("Failed to get module: {}", module_name));
    }
    Ok(module)
}

// Utility to get exported function from il2cpp dll
pub fn resolve_function_ptr_from_name(
    module: HMODULE,
    name: &str,
) -> Result<Option<FARPROC>, String> {
    if module.is_invalid() {
        return Err(format!("Invalid module handle"));
    }

    let c_name =
        CString::new(name).map_err(|e| format!("Failed to create CString for name: {}", e))?;

    // Explicitly convert the *const i8 to PCSTR
    let proc_address = unsafe {
        let addr = GetProcAddress(module, PCSTR::from_raw(c_name.as_ptr() as *const u8));
        addr
    };
    if proc_address.is_some() {
        Ok(Some(proc_address))
    } else {
        Ok(None)
    }
}
