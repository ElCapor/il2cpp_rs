
use crate::{il2cpp::classes::unity_object::UnityObjectInner, il2cpp_view};

il2cpp_view! {
    pub struct GameObject {
        pub obj: UnityObjectInner,
    }
}
pub type GameObject<'a> = GameObjectView<'a>;

impl<'a> GameObject<'a> {}

// Now you have:
// GameObjectInner = raw struct
// GameObjectView<'a> = zero-cost lifetime view
