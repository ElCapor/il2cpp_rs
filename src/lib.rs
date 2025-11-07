pub mod console;
pub mod il2cpp;

use il2cpp::unityresolve::{IL2CPP_API, init_unity_resolve};
use std::process::exit;
use std::thread;

use crate::console::wait_line_press_to_exit;

pub fn entry_point() {
    // Initialize the console
    if let Err(e) = console::allocate_console() {
        println!("Error: {}", e);
        exit(-1);
    }
    println!("Initializing Il2CppApi...");
    match init_unity_resolve() {
        Ok(api) => api,
        Err(e) => {
            println!("Error: {}", e);
            wait_line_press_to_exit(-1);
        }
    };
    println!("Initializing Il2CppApi... Done");
    IL2CPP_API
        .get()
        .map(|api| api.get_api().print_all_function_ptrs());
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
