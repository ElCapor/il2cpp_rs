use crate::{il2cpp::classes::object::{ObjectInner}, il2cpp_view};



il2cpp_view! {
    pub struct UnityObject {
        pub obj: ObjectInner,
        pub m_cached_ptr: *mut std::ffi::c_void,
    }
}

pub type UnityObject<'a> = UnityObjectView<'a>;
