use crate::{
    il2cpp::classes::{
        object::Object,
        string::{UnityString, UnityStringInner},
    },
    il2cpp_cache,
};
use std::{marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct UnityObjectInner {
    pub obj: Object,                         // inherited fields from System.Object
    pub m_cached_ptr: *mut std::ffi::c_void, // UnityEngine internal native object
}

/// A zero-cost, lifetime-bound view over a UnityEngine.Object
#[derive(Copy, Clone)]
pub struct UnityObject<'a> {
    ptr: NonNull<UnityObjectInner>,
    _marker: std::marker::PhantomData<&'a UnityObjectInner>,
}

impl<'a> UnityObject<'a> {
    /// Create a view from a raw pointer
    pub fn from_ptr(ptr: *mut UnityObjectInner) -> Option<Self> {
        NonNull::new(ptr).map(|nn| Self {
            ptr: nn,
            _marker: PhantomData,
        })
    }

    /// From a reference — allows `&*mut UnityObject` or `&UnityObject`
    #[inline(always)]
    pub fn from_ref(r: &'a UnityObjectInner) -> Self {
        let ptr = NonNull::from(r);
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Get the managed address (Il2CppObject*)
    #[inline(always)]
    pub fn managed_ptr(&self) -> *mut UnityObjectInner {
        self.ptr.as_ptr()
    }

    /// Get the native UnityEngine object pointer
    #[inline(always)]
    pub fn cached_ptr(&self) -> *mut std::ffi::c_void {
        unsafe { self.ptr.as_ref().m_cached_ptr }
    }

    /// Get a reference to the underlying `UnityObjectInner`
    #[inline(always)]
    pub fn as_ref(&self) -> &'a UnityObjectInner {
        unsafe { self.ptr.as_ref() }
    }

    /// Convert to `*mut Il2CppObject` (if your Object type alias is used for managed calls)
    #[inline(always)]
    pub fn as_il2cpp_object(&self) -> *mut crate::il2cpp::classes::object::Object {
        self.ptr.as_ptr() as *mut _
    }

    pub fn get_name(&self, cache: &il2cpp_cache::Cache) -> Result<UnityString<'a>, String> {
        type GetNameMethod =
            unsafe extern "C" fn(this: *mut UnityObjectInner) -> *mut UnityStringInner;

        let get_name_method = cache
            .get_assembly("UnityEngine.CoreModule.dll")
            .expect("Failed to get UnityEngine.CoreModule.dll")
            .get("Object")
            .expect("Failed to get Object")
            .get_method("get_name", Vec::new())
            .expect("Failed to find get_name method")
            .try_callable::<GetNameMethod>()
            .expect("Failed to cast get_name to GetNameMethod");

        let name = unsafe { get_name_method(self.managed_ptr()) };
        Ok(UnityString::from_ptr(name).expect("Failed to build string view"))
    }
}
