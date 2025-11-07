use std::io::{self};
use windows::Win32::System::Console::AllocConsole;
pub fn allocate_console() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        AllocConsole()?;
    }
    Ok(())
}

pub fn wait_line() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

pub fn instructuted_wait_line(instruction: String) {
    println!("{}", instruction);
    wait_line();
}

pub fn wait_line_press_to_exit(error_code: i32) {
    instructuted_wait_line(String::from("Press enter to exit...."));
    std::process::exit(error_code);
}
