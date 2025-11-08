pub struct Assembly {
    address: *mut u8,
    name: String,
    file: String,
}

impl Assembly {
    pub fn new(address: *mut u8, name: String, file: String) -> Self {
        Self {
            address,
            name,
            file,
        }
    }

    pub fn cache_classes(&mut self) {}
}
