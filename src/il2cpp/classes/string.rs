use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::NonNull};

use crate::il2cpp::classes::object::Object;

/// Representation of System.String in IL2CPP
#[repr(C)]
pub struct Il2CppString {
    pub obj: Object,             // inherited object
    pub m_string_length: i32,    // length in characters
    pub m_first_char: [u16; 32], // inline buffer, UTF-16
}

impl Il2CppString {
    /// Create a reference from a raw pointer returned by IL2CPP
    pub fn from_ptr<'a>(ptr: *mut Il2CppString) -> Option<&'a Self> {
        unsafe { NonNull::new(ptr).map(|nn| nn.as_ref()) }
    }

    /// Mutable version if you need to modify the string (rare)
    pub fn from_ptr_mut<'a>(ptr: *mut Il2CppString) -> Option<&'a mut Self> {
        unsafe { NonNull::new(ptr).map(|mut nn| nn.as_mut()) }
    }

    /// Convert to Rust `String` (UTF-16 -> UTF-8)
    pub fn to_string(&self) -> String {
        let slice = &self.m_first_char[..self.m_string_length as usize];
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }

    /// Index into the string
    pub fn char_at(&self, i: usize) -> Option<u16> {
        if i >= self.m_string_length as usize {
            None
        } else {
            Some(self.m_first_char[i])
        }
    }
}

pub type UnityString = Il2CppString;
