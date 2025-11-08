use super::c_types::*;

// Il2Cpp lowvel api function pointer types
pub type Il2CppInitFn = unsafe extern "C" fn();
pub type Il2CppShutdownFn = unsafe extern "C" fn();
pub type Il2CppGetDomainFn = unsafe extern "C" fn() -> Il2CppDomain;
pub type Il2CppThreadAttachFn = unsafe extern "C" fn(domain: Il2CppDomain);
pub type Il2CppThreadDetachFn = unsafe extern "C" fn(domain: Il2CppDomain);
pub type Il2CppAssemblyGetImageFn = unsafe extern "C" fn(assembly: Il2CppAssembly) -> Il2CppImage;
pub type Il2CppClassGetNameFn = unsafe extern "C" fn(class: Il2CppClass) -> *const i8;
pub type Il2CppClassGetNamespaceFn = unsafe extern "C" fn(class: Il2CppClass) -> *const i8;
pub type Il2CppClassGetParentFn = unsafe extern "C" fn(class: Il2CppClass) -> Il2CppClass;
pub type Il2CppClassFromNameFn =
    unsafe extern "C" fn(image: Il2CppImage, namespace: *const i8, name: *const i8) -> Il2CppClass;
pub type Il2CppClassGetMethodsFn =
    unsafe extern "C" fn(klass: Il2CppClass, iter: *mut usize) -> Il2CppMethodInfo;
pub type Il2CppMethodGetNameFn = unsafe extern "C" fn(method: Il2CppMethodInfo) -> *const i8;
pub type Il2CppDomainGetAssembliesFn =
    unsafe extern "C" fn(domain: Il2CppDomain, size: *mut usize) -> *mut Il2CppAssembly;
pub type Il2CppImageGetNameFn = unsafe extern "C" fn(image: Il2CppImage) -> *const i8;
pub type Il2CppImageGetFileNameFn = unsafe extern "C" fn(image: Il2CppImage) -> *const i8;
pub type Il2CppImageGetClassCountFn = unsafe extern "C" fn(image: Il2CppImage) -> usize;
pub type Il2CppImageGetClassFn =
    unsafe extern "C" fn(image: Il2CppImage, index: usize) -> Il2CppClass;
pub type Il2CppClassGetFieldsFn =
    unsafe extern "C" fn(klass: Il2CppClass, iter: *mut *mut u8) -> *mut u8; // Il2CppField
pub type Il2CppFieldGetNameFn = unsafe extern "C" fn(field: *mut u8) -> *const i8;
pub type Il2CppFieldGetOffsetFn = unsafe extern "C" fn(field: *mut u8) -> u32;
pub type Il2CppMethodGetParamCountFn = unsafe extern "C" fn(method: Il2CppMethodInfo) -> u32;
pub type Il2CppMethodGetParamNameFn =
    unsafe extern "C" fn(method: Il2CppMethodInfo, index: u32) -> *const i8;
pub type Il2CppMethodGetReturnTypeFn = unsafe extern "C" fn(method: Il2CppMethodInfo) -> Il2CppType;
pub type Il2CppTypeGetNameFn = unsafe extern "C" fn(itype: Il2CppType) -> *const i8;
