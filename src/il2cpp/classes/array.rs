use crate::il2cpp::classes::object::Object;
use std::marker::PhantomData;
use std::slice;

#[repr(C)]
pub struct Il2CppArrayBounds {
    pub length: usize,
    pub lower_bound: i32,
}

#[repr(C)]
pub struct Il2CppArrayHeader {
    pub obj: Object,
    pub bounds: *mut Il2CppArrayBounds,
    pub max_length: u32,
}

pub struct Il2CppArray<T> {
    ptr: *mut Il2CppArrayHeader,
    phantom: PhantomData<T>,
}

impl<T> Il2CppArray<T> {
    /// Construct wrapper from raw pointer
    pub fn from_ptr(ptr: *mut Il2CppArrayHeader) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    /// Number of elements
    pub fn len(&self) -> usize {
        unsafe { (*self.ptr).max_length as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Immutable reference to element at index
    pub fn at(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(&*self.data_ptr().add(index)) }
    }

    /// Mutable reference to element at index
    pub fn at_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(&mut *self.data_ptr_mut().add(index)) }
    }

    /// Slice over the array (immutable)
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data_ptr(), self.len()) }
    }

    /// Slice over the array (mutable)
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data_ptr_mut(), self.len()) }
    }

    /// Iterator (immutable)
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Iterator (mutable)
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Internal: pointer to first element
    unsafe fn data_ptr(&self) -> *const T {
        unsafe { (self.ptr as *const u8).add(std::mem::size_of::<Il2CppArrayHeader>()) as *const T }
    }

    unsafe fn data_ptr_mut(&mut self) -> *mut T {
        unsafe { (self.ptr as *mut u8).add(std::mem::size_of::<Il2CppArrayHeader>()) as *mut T }
    }
}

pub type Array<T> = Il2CppArray<T>;
