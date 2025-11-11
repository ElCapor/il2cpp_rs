use crate::{il2cpp::classes::object::ObjectInner, il2cpp_view_generic};
use std::{marker::PhantomData, slice};

#[repr(C)]
pub struct Il2CppArrayBounds {
    pub length: usize,
    pub lower_bound: i32,
}

il2cpp_view_generic! {
    pub struct Il2CppArray<T> {
        pub obj: ObjectInner,
        pub bounds: *mut Il2CppArrayBounds,
        pub max_length: u32,
        _phantom: PhantomData<T>,
    }
}

impl<'a, T> Il2CppArrayView<'a, T> {
    /// Number of elements
    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { (self.ptr.as_ref()).max_length as usize }
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
        unsafe {
            (self.ptr.as_ptr() as *const u8).add(std::mem::size_of::<Il2CppArrayInner<T>>())
                as *const T
        }
    }

    /// Get raw header
    #[inline(always)]
    pub fn header(&self) -> &'a Il2CppArrayInner<T> {
        unsafe { &*self.ptr.as_ptr() }
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
            let ptr = (self.ptr.as_ptr() as *mut u8).add(std::mem::size_of::<Il2CppArrayInner<T>>())
                as *mut T;
            std::slice::from_raw_parts_mut(ptr, len).iter_mut()
        }
    }
}

pub type ArrayInner<T> = Il2CppArrayInner<T>;
pub type Array<'a, T> = Il2CppArrayView<'a, T>;
