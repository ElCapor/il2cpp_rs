use crate::{il2cpp::classes::object::ObjectInner, il2cpp_view};
use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::NonNull};

il2cpp_view! {
    pub struct Il2CppString {
        pub obj: ObjectInner,
        pub m_string_length: i32,
        pub m_first_char: [u16; 32],
    }
}

impl<'a> Il2CppStringView<'a> {
    /// Length in characters
    pub fn len(&self) -> usize {
        unsafe { self.ptr.as_ref().m_string_length as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Index into the string (UTF-16 code unit)
    pub fn char_at(&self, idx: usize) -> Option<u16> {
        if idx >= self.len() {
            None
        } else {
            unsafe { Some(self.ptr.as_ref().m_first_char[idx]) }
        }
    }

    /// Convert to a Rust `String` (UTF-16 -> UTF-8) if needed
    pub fn to_string(&self) -> String {
        let slice = unsafe { &self.ptr.as_ref().m_first_char[..self.len()] };
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

pub type UnityStringInner = Il2CppStringInner;
pub type UnityString<'a> = Il2CppStringView<'a>;
