use crate::il2cpp::classes::typ::Type;

pub struct Field {
    address: *mut u8,
    name: String,
    typ: Type,
}
