pub mod console;
pub mod il2cpp;

use il2cpp::unityresolve::IL2CPP_API;
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
    let mut api = IL2CPP_API.lock();
    //api.get_api().unwrap().print_all_function_ptrs();

    match api.init() {
        Ok(_) => {
            println!("Initializing Il2CppApi... Done");
        }
        Err(e) => {
            println!("Failed init {}", e);
        }
    }

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
