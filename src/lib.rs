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

    println!("Initializing Il2CppApi... Done");
    if let Some(api) = IL2CPP_API.get_mut() {
        match api.init() {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
                wait_line_press_to_exit(-1);
            }
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
