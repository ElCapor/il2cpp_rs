use crate::il2cpp::classes::array::ArrayInner;
use crate::il2cpp_cache::Il2CppCacheTrait;
use crate::il2cpp_view;
use crate::{
    il2cpp::{
        classes::{
            array::Array,
            string::{UnityString, UnityStringInner},
        },
        il2cpp_sys::c_types::Il2CppClass,
    },
    il2cpp_cache,
};

#[repr(C)]
pub struct MonitorData {
    _unused: [u8; 0],
}

il2cpp_view! {
    pub struct Object {
        pub klass: Il2CppClass,
        pub monitor: *mut MonitorData,
    }
}
impl<'a> ObjectView<'a> {
    /// Get the class pointer (type info)
    #[inline(always)]
    pub fn klass(&self) -> *const Il2CppClass {
        &self.as_ref().klass
    }

    /// Get the monitor (sync lock object)
    #[inline(always)]
    pub fn monitor(&self) -> *mut MonitorData {
        self.as_ref().monitor
    }

    pub fn get_name(&self, cache: &il2cpp_cache::Cache) -> Result<UnityString<'a>, String> {
        type GetNameMethod = unsafe extern "C" fn(this: *mut ObjectInner) -> *mut UnityStringInner;

        let get_name_method = cache
            .get_assembly("UnityEngine.CoreModule.dll")
            .expect("Failed to get UnityEngine.CoreModule.dll")
            .get("Object")
            .expect("Failed to get Object")
            .get_method_with_args("get_name", Vec::new())
            .expect("Failed to find get_name method")
            .try_callable::<GetNameMethod>()
            .expect("Failed to cast get_name to GetNameMethod");

        let name = unsafe { get_name_method(self.as_ptr()) };
        Ok(UnityString::from_ptr(name).expect("Failed to build string view"))
    }

    pub fn find_objects_of_type(
        cache: &impl Il2CppCacheTrait,
        obj_type: Object,
        include_inactve: bool,
    ) -> Vec<Object<'a>> {
        type FindObjectsOfTypeFn = unsafe extern "C" fn(
            obj_type: *mut ObjectInner,
            include_inactve: bool,
        )
            -> *mut ArrayInner<*mut ObjectInner>;

        let mut arg_types = Vec::new();
        arg_types.push("System.Type");
        arg_types.push("System.Boolean");

        let find_objects_of_type = cache
            .get_assembly("UnityEngine.CoreModule.dll")
            .expect("Failed to get UnityEngine.CoreModule.dll")
            .get("Object")
            .expect("Failed to get Object")
            .get_method_with_args("FindObjectsOfType", arg_types)
            .expect("Failed to find FindObjectsOfType method")
            .try_callable::<FindObjectsOfTypeFn>()
            .expect("Failed to cast FindObjectsOfType to FindObjectsOfTypeFn");

        let array = unsafe { find_objects_of_type(obj_type.as_ptr(), include_inactve) };
        let array_view =
            Array::<*mut ObjectInner>::from_ptr(array).expect("Failed to build array view");
        array_view
            .into_iter()
            .map(|ptr| Object::from_ptr(*ptr).unwrap())
            .collect()
    }
}

pub type Object<'a> = ObjectView<'a>;
