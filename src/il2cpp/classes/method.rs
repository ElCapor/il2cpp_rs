use super::class::Class;
use super::itype::Type;
#[derive(Clone)]
pub struct Method {
    address: *mut u8,
    name: String,
    class: Class,
    return_type: Type,
    flags: i32,
    static_methodon: bool,
    function: *mut u8,
}

impl Method {
    pub fn new(
        address: *mut u8,
        name: String,
        class: Class,
        return_type: Type,
        flags: i32,
        static_methodon: bool,
        function: *mut u8,
    ) -> Self {
        Self {
            address,
            name,
            class,
            return_type,
            flags,
            static_methodon,
            function,
        }
    }
}
