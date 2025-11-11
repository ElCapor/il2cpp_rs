use crate::{
    il2cpp::classes::{
        object::{Object, ObjectInner, ObjectView},
        string::UnityString,
    },
    il2cpp_cache,
};

// Special trait for views of Il2Cpp objects
// THIS IS ONLY FOR IL2CPP OBJECTS with an obj field
pub trait Il2CppView<'a, Inner> {
    fn from_ptr(ptr: *mut Inner) -> Option<Self>
    where
        Self: Sized;
    fn from_ref(r: &'a Inner) -> Self
    where
        Self: Sized;
    fn as_ptr(&self) -> *mut Inner;
    fn as_ref(&self) -> &'a Inner;

    fn as_il2cpp_object(&self) -> *mut ObjectInner;
}

// Extension trait to provide unchecked zero-cost casts between views
pub trait Il2CppViewCast<'a, Inner>: Il2CppView<'a, Inner> + Sized {
    /// Reinterpret this view as another view type without runtime checks.
    /// Safety: caller must ensure the underlying dynamic object is layout-compatible with DInner.
    fn cast<DInner, D>(&self) -> D
    where
        DInner: Sized,
        D: Il2CppView<'a, DInner>,
    {
        D::from_ptr(self.as_ptr() as *mut DInner).expect("Failed to build destination view")
    }

    /// Reinterpret a reference to this view as another view type without runtime checks.
    /// Safety: caller must ensure the underlying dynamic object is layout-compatible with DInner.
    fn cast_ref<DInner, D>(&self) -> D
    where
        DInner: Sized,
        D: Il2CppView<'a, DInner>,
    {
        D::from_ptr(self.as_ptr() as *mut DInner).expect("Failed to build destination view")
    }
}

impl<'a, Inner, T> Il2CppViewCast<'a, Inner> for T where T: Il2CppView<'a, Inner> {}

pub trait Il2CppViewGetName<'a, Inner>: Il2CppViewCast<'a, Inner> {
    fn get_name(&self, cache: &il2cpp_cache::Cache) -> Result<UnityString<'a>, String> {
        self.cast::<ObjectInner, ObjectView>().get_name(cache)
    }
}

impl<'a, Inner, T> Il2CppViewGetName<'a, Inner> for T where T: Il2CppViewCast<'a, Inner> {}

// Macro to declare a C-compatible inner struct and generate its zero-cost View
#[macro_export]
macro_rules! il2cpp_view {
    (
        $(#[$m:meta])* $vis:vis struct $Name:ident {
            $( $field_vis:vis $field_name:ident : $field_ty:ty, )* $(,)?
        }
    ) => {
        ::paste::paste! {
            #[repr(C)]
            $(#[$m])* $vis struct [<$Name Inner>] {
                $( $field_vis $field_name : $field_ty, )*
            }

            #[derive(Copy, Clone)]
            $vis struct [<$Name View>]<'a> {
                ptr: ::std::ptr::NonNull<[<$Name Inner>]>,
                _marker: ::std::marker::PhantomData<&'a [<$Name Inner>]>,
            }

            impl<'a> [<$Name View>]<'a> {
                #[inline(always)]
                pub fn from_ptr(ptr: *mut [<$Name Inner>]) -> Option<Self> {
                    ::std::ptr::NonNull::new(ptr).map(|nn| Self { ptr: nn, _marker: ::std::marker::PhantomData })
                }

                #[inline(always)]
                pub fn from_ref(r: &'a [<$Name Inner>]) -> Self {
                    Self { ptr: ::std::ptr::NonNull::from(r), _marker: ::std::marker::PhantomData }
                }

                #[inline(always)]
                pub fn as_ptr(&self) -> *mut [<$Name Inner>] { self.ptr.as_ptr() }

                #[inline(always)]
                pub fn as_ref(&self) -> &'a [<$Name Inner>] { unsafe { self.ptr.as_ref() } }

                #[inline(always)]
                pub fn as_il2cpp_object(&self) -> *mut crate::il2cpp::classes::object::ObjectInner { self.ptr.as_ptr() as *mut _ }
            }

            impl<'a> crate::il2cpp::classes::il2cpp_view::Il2CppView<'a, [<$Name Inner>]> for [<$Name View>]<'a> {
                #[inline(always)]
                fn from_ptr(ptr: *mut [<$Name Inner>]) -> Option<Self> { <Self>::from_ptr(ptr) }

                #[inline(always)]
                fn from_ref(r: &'a [<$Name Inner>]) -> Self { <Self>::from_ref(r) }

                #[inline(always)]
                fn as_ptr(&self) -> *mut [<$Name Inner>] { <Self>::as_ptr(self) }

                #[inline(always)]
                fn as_ref(&self) -> &'a [<$Name Inner>] { <Self>::as_ref(self) }

                #[inline(always)]
                fn as_il2cpp_object(&self) -> *mut crate::il2cpp::classes::object::ObjectInner { self.ptr.as_ptr() as *mut _ }
            }
        }
    };
}
