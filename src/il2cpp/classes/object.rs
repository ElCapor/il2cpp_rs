use crate::{
    il2cpp::{
        classes::{
            array::{Array, ArrayInner},
            string::{UnityString, UnityStringInner},
        },
        il2cpp_sys::c_types::Il2CppClass,
    },
    il2cpp_cache,
};
use std::{marker::PhantomData, ptr::NonNull};

#[repr(C)]
#[derive(Debug)]
pub struct MonitorData {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ObjectInner {
    pub klass: Il2CppClass,
    pub monitor: *mut MonitorData,
}

/// A lifetime-tracked, zero-cost view over an ObjectInner
#[derive(Copy, Clone)]
pub struct ObjectView<'a> {
    ptr: NonNull<ObjectInner>,
    _marker: PhantomData<&'a ObjectInner>,
}

impl<'a> ObjectView<'a> {
    /// Create from a raw pointer
    #[inline(always)]
    pub fn from_ptr(ptr: *mut ObjectInner) -> Option<Self> {
        NonNull::new(ptr).map(|nn| Self {
            ptr: nn,
            _marker: PhantomData,
        })
    }

    /// Create from a reference (already dereferenced)
    #[inline(always)]
    pub fn from_ref(r: &'a ObjectInner) -> Self {
        Self {
            ptr: NonNull::from(r),
            _marker: PhantomData,
        }
    }

    /// Get the raw pointer back
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut ObjectInner {
        self.ptr.as_ptr()
    }

    /// Immutable access
    #[inline(always)]
    pub fn as_ref(&self) -> &'a ObjectInner {
        unsafe { self.ptr.as_ref() }
    }

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
            .get_method("get_name", Vec::new())
            .expect("Failed to find get_name method")
            .try_callable::<GetNameMethod>()
            .expect("Failed to cast get_name to GetNameMethod");

        let name = unsafe { get_name_method(self.as_ptr()) };
        Ok(UnityString::from_ptr(name).expect("Failed to build string view"))
    }

    pub fn find_objects_of_type(
        cache: &il2cpp_cache::Cache,
        obj_type: Object,
        include_inactve: bool,
    ) -> Vec<Object<'a>> {
        type FindObjectsOfTypeFn = unsafe extern "C" fn(
            obj_type: *mut ObjectInner,
            include_inactve: bool,
        ) -> *mut ArrayInner;

        let mut arg_types = Vec::new();
        arg_types.push("System.Type");
        arg_types.push("System.Boolean");

        let find_objects_of_type = cache
            .get_assembly("UnityEngine.CoreModule.dll")
            .expect("Failed to get UnityEngine.CoreModule.dll")
            .get("Object")
            .expect("Failed to get Object")
            .get_method("FindObjectsOfType", arg_types)
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
