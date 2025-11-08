use std::sync::Arc;

use crate::il2cpp::classes::itype::Type;
use crate::il2cpp::classes::itype::TypeInner;

#[derive(Debug)]
pub struct ArgInner {
    pub name: String,
    pub itype: Type,
}

// Public handle used throughout the model
pub type Arg = Arc<ArgInner>;

impl ArgInner {
    pub fn default() -> Arg {
        Arc::new(Self {
            name: "".to_string(),
            itype: TypeInner::default(),
        })
    }

    pub fn new(name: String, type_: Type) -> Arg {
        Arc::new(Self { name, itype: type_ })
    }
}
