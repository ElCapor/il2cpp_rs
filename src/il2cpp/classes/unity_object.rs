use crate::{
    il2cpp::classes::{
        object::{ObjectInner, ObjectView},
        string::{UnityString, UnityStringInner},
    },
    il2cpp_cache,
};
use std::{marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct UnityObjectInner {
    pub obj: ObjectInner,                    // inherited fields from System.Object
    pub m_cached_ptr: *mut std::ffi::c_void, // UnityEngine internal native object
}

/// A zero-cost, lifetime-bound view over a UnityEngine.Object
#[derive(Copy, Clone)]
pub struct UnityObjectView<'a> {
    ptr: NonNull<UnityObjectInner>,
    _marker: std::marker::PhantomData<&'a UnityObjectInner>,
}

impl<'a> UnityObjectView<'a> {
    /// Create a view from a raw pointer
    pub fn from_ptr(ptr: *mut UnityObjectInner) -> Option<Self> {
        NonNull::new(ptr).map(|nn| Self {
            ptr: nn,
            _marker: PhantomData,
        })
    }

    /// From a reference — allows `&*mut UnityObjectView` or `&UnityObjectView`
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
    pub fn as_il2cpp_object(&self) -> *mut ObjectInner {
        self.ptr.as_ptr() as *mut _
    }
}

impl From<*mut ObjectInner> for UnityObjectView<'_> {
    fn from(ptr: *mut ObjectInner) -> Self {
        Self::from_ptr(ptr as _).expect("Failed to build object view")
    }
}

impl From<ObjectView<'_>> for UnityObjectView<'_> {
    fn from(obj: ObjectView<'_>) -> Self {
        Self::from_ptr(obj.as_ptr() as _).expect("Failed to build object view")
    }
}

impl From<&ObjectView<'_>> for UnityObjectView<'_> {
    fn from(obj: &ObjectView<'_>) -> Self {
        Self::from_ptr(obj.as_ptr() as _).expect("Failed to build object view")
    }
}

pub type UnityObject<'a> = UnityObjectView<'a>;
