pub mod console;
pub mod il2cpp;

use il2cpp::il2cpp::Il2Cpp;
use il2cpp::il2cpp_sys::Il2CppApi;
use std::process::exit;
use std::sync::OnceLock;
use std::thread;

use crate::console::wait_line_press_to_exit;

static IL2CPP_API: OnceLock<Il2CppApi> = OnceLock::new();

pub fn init_il2cpp_api() -> Result<(), String> {
    let mut api = Il2CppApi::new();
    unsafe {
        if let Err(e) = api.initialize("GameAssembly.dll") {
            return Err(format!("Failed to initialize Il2CppApi: {}", e));
        }
    }
    let ret = IL2CPP_API.set(api);
    if ret.is_err() {
        Err("Failed to initialize Il2CppApi".to_string())
    } else {
        Ok(())
    }
}

pub fn get_il2cpp_api_safe() -> Option<Il2Cpp<'static>> {
    unsafe {
        IL2CPP_API
            .get()
            .map(|api_instance| Il2Cpp::new(api_instance))
    }
}

pub fn entry_point() {
    // Initialize the console
    if let Err(e) = console::allocate_console() {
        println!("Error: {}", e);
        exit(-1);
    }
    println!("Initializing Il2CppApi...");
    match init_il2cpp_api() {
        Ok(api) => api,
        Err(e) => {
            println!("Error: {}", e);
            wait_line_press_to_exit(-1);
        }
    };
    println!("Initializing Il2CppApi... Done");
    IL2CPP_API.get().map(|api| api.print_all_function_ptrs());
    wait_line_press_to_exit(-1);
}

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_: usize, reason: u32, _: usize) -> i32 {
    match reason {
        1 => {
            let _ = thread::spawn(|| {
                entry_point();
            });
        }
        _ => {}
    }
    1
}
