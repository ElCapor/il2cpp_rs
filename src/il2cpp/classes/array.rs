use crate::il2cpp::classes::object::ObjectInner;
use std::{marker::PhantomData, slice};

#[repr(C)]
pub struct Il2CppArrayBounds {
    pub length: usize,
    pub lower_bound: i32,
}

#[repr(C)]
pub struct Il2CppArrayHeader {
    pub obj: ObjectInner,
    pub bounds: *mut Il2CppArrayBounds,
    pub max_length: u32,
}

/// Lifetime-tracked, zero-cost view over an Il2Cpp array
pub struct Il2CppArrayView<'a, T> {
    ptr: *mut Il2CppArrayHeader,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Il2CppArrayView<'a, T> {
    /// Construct wrapper from raw pointer
    pub fn from_ptr(ptr: *mut Il2CppArrayHeader) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                _marker: PhantomData,
            })
        }
    }

    /// Number of elements
    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { (*self.ptr).max_length as usize }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Immutable reference to element at index
    pub fn at(&self, index: usize) -> Option<&'a T> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(&*self.data_ptr().add(index)) }
    }

    /// Slice over the array (immutable)
    pub fn as_slice(&self) -> &'a [T] {
        unsafe { slice::from_raw_parts(self.data_ptr(), self.len()) }
    }

    /// Iterator
    pub fn iter(&self) -> slice::Iter<'a, T> {
        self.as_slice().iter()
    }

    /// Internal pointer to first element
    #[inline(always)]
    unsafe fn data_ptr(&self) -> *const T {
        unsafe { (self.ptr as *const u8).add(std::mem::size_of::<Il2CppArrayHeader>()) as *const T }
    }

    /// Get raw header
    #[inline(always)]
    pub fn header(&self) -> &'a Il2CppArrayHeader {
        unsafe { &*self.ptr }
    }
}

impl<'a, T> IntoIterator for &'a Il2CppArrayView<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
// this will be handy for hooking , but needs to be tested at some point
impl<'a, T> IntoIterator for &'a mut Il2CppArrayView<'a, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let len = self.len();
            let ptr = (self.ptr as *mut u8).add(std::mem::size_of::<Il2CppArrayHeader>()) as *mut T;
            std::slice::from_raw_parts_mut(ptr, len).iter_mut()
        }
    }
}

pub type ArrayInner = Il2CppArrayHeader;
pub type Array<'a, T> = Il2CppArrayView<'a, T>;
