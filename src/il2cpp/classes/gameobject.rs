use crate::{
    il2cpp::{
        class_get_type,
        classes::{
            il2cpp_view::{Il2CppViewCast, Ptr2View},
            object::{ObjectInner, ObjectView},
            unity_object::UnityObjectInner,
        },
        type_get_object,
    },
    il2cpp_cache, il2cpp_view,
};

il2cpp_view! {
    pub struct GameObject {
        pub obj: UnityObjectInner,
    }
}
pub type GameObject<'a> = GameObjectView<'a>;

impl<'a> GameObject<'a> {
    pub fn get_all_gameobjects(cache: &il2cpp_cache::Cache) -> Vec<GameObjectView<'a>> {
        let core_module = cache
            .get_assembly("UnityEngine.CoreModule.dll")
            .expect("Failed to get core module");
        let gameobject_class = core_module
            .get("GameObject")
            .expect("Failed to get gameobject class");
        let game_object_type = class_get_type(gameobject_class.address)
            .expect("Failed to get type for gameobject class");
        let game_object_type_obj = type_get_object(game_object_type)
            .expect("Failed to get object for gameobject type")
            as *mut ObjectInner;

        ObjectView::find_objects_of_type(cache, game_object_type_obj.view(), true)
            .into_iter()
            .map(|obj| obj.cast::<GameObjectInner, GameObjectView>())
            .collect()
    }
}

// Now you have:
// GameObjectInner = raw struct
// GameObjectView<'a> = zero-cost lifetime view
