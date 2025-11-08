#[derive(Clone)]
pub struct Type {
    address: *mut u8,
    name: String,
    size: usize,
}

impl Type {
    pub fn default() -> Self {
        Self {
            address: std::ptr::null_mut(),
            name: "".to_string(),
            size: 0,
        }
    }

    pub fn new(address: *mut u8, name: String, size: usize) -> Self {
        Self {
            address,
            name,
            size,
        }
    }
}
