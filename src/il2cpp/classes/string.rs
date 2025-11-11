use crate::il2cpp::classes::object::Object;
use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::NonNull};

#[repr(C)]
pub struct Il2CppString {
    pub obj: Object,
    pub m_string_length: i32,
    pub m_first_char: [u16; 32],
}

/// A zero-cost view over an IL2CPP string
pub struct Il2CppStr<'a> {
    inner: &'a Il2CppString,
}

impl Il2CppStr<'_> {
    /// Create a view from a raw pointer
    pub fn from_ptr<'a>(ptr: *mut Il2CppString) -> Option<Il2CppStr<'a>> {
        unsafe { NonNull::new(ptr).map(|nn| Il2CppStr { inner: nn.as_ref() }) }
    }

    /// Length in characters
    pub fn len(&self) -> usize {
        self.inner.m_string_length as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Index into the string (UTF-16 code unit)
    pub fn char_at(&self, idx: usize) -> Option<u16> {
        if idx >= self.len() {
            None
        } else {
            Some(self.inner.m_first_char[idx])
        }
    }

    /// Convert to a Rust `String` (UTF-16 -> UTF-8) if needed
    pub fn to_string(&self) -> String {
        let slice = &self.inner.m_first_char[..self.len()];
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }

    /// Borrow the underlying IL2CPP string directly
    pub fn as_raw(&self) -> &Il2CppString {
        self.inner
    }
}

pub type UnityStringInner = Il2CppString;
pub type UnityString<'a> = Il2CppStr<'a>;
