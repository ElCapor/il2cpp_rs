use crate::{
    il2cpp::classes::{
        il2cpp_view::{Il2CppViewCast, Ptr2View},
        object::{ObjectInner, ObjectView},
        unity_object::UnityObjectInner,
    },
    il2cpp_cache::Il2CppCacheTrait,
    il2cpp_view,
};

il2cpp_view! {
    pub struct GameObject {
        pub obj: UnityObjectInner,
    }
}
pub type GameObject<'a> = GameObjectView<'a>;

impl<'a> GameObject<'a> {
    pub fn get_all_gameobjects(cache: &impl Il2CppCacheTrait) -> Vec<GameObjectView<'a>> {
        let game_object_type_obj = cache
            .get_assembly("UnityEngine.CoreModule.dll")
            .expect("Failed to get core module")
            .get("GameObject")
            .expect("Failed to get gameobject class")
            .get_type_object()
            .expect("Failed to get object for gameobject type")
            as *mut ObjectInner;

        ObjectView::find_objects_of_type(cache, game_object_type_obj.view(), true)
            .into_iter()
            .map(|obj| obj.cast::<GameObjectInner, GameObjectView>())
            .collect()
    }
}
