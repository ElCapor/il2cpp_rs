use crate::{il2cpp::classes::unity_object::UnityObjectInner, il2cpp_view};

il2cpp_view! {
    pub struct Component {
        pub obj: UnityObjectInner,
    }
}
