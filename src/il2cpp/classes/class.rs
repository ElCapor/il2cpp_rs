use crate::il2cpp::classes::{field::Field, method::Method};

pub struct Class {
    address: *mut u8,
    name: String,
    parent: String,
    namespace: String,
    fields: Vec<Field>,
    methods: Vec<Method>,
}
