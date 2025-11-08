pub mod console;
pub mod il2cpp;

use std::process::exit;
use std::thread;

use crate::console::wait_line_press_to_exit;
use crate::il2cpp::{assembly_get_image, domain_get_assemblies, image_get_name, thread_attach};

pub fn entry_point() {
    // Initialize the console
    if let Err(e) = console::allocate_console() {
        println!("Error: {}", e);
        exit(-1);
    }
    println!("Initializing Il2CppApi...");
    match il2cpp::init("GameAssembly.dll") {
        Ok(_) => {
            println!("Il2CppApi initialized");
        }
        Err(e) => {
            println!("Error: {}", e);
            wait_line_press_to_exit(-1);
        }
    }

    match il2cpp::get_domain() {
        Ok(domain) => {
            println!("Domain: {:p}", domain);
            let _ = thread_attach(domain);
            println!("Attached to domain");
            let assemblies = domain_get_assemblies(domain).unwrap();
            for assembly in assemblies {
                if let Ok(image) = assembly_get_image(assembly) {
                    let name = image_get_name(image);
                    if (name.is_err()) {
                        continue;
                    }
                    println!("Found assembly {}", name.unwrap());
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            wait_line_press_to_exit(-1);
        }
    }

    il2cpp::print_all_function_ptrs();
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
