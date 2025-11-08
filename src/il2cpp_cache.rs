use crate::il2cpp::{
    assembly_get_image, class_get_fields, class_get_methods, class_get_name, class_get_namespace,
    class_get_parent,
    classes::{
        arg::ArgInner,
        assembly::Assembly,
        class::{Class, ClassInner},
        field::FieldInner,
        itype::TypeInner,
        method::MethodInner,
    },
    domain_get_assemblies, field_get_name, field_get_offset, field_get_type,
    il2cpp_sys::c_types::{Il2CppDomain, Il2CppImage},
    image_get_class, image_get_class_count, image_get_filename, image_get_name, method_get_flags,
    method_get_name, method_get_param, method_get_param_count, method_get_param_name,
    method_get_return_type, type_get_name,
};

use std::ffi::c_void;
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, RwLock},
};

pub struct Cache {
    assemblies: Vec<Assembly>,
}
impl Cache {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            assemblies: Vec::new(),
        }
    }

    pub fn parse_assemblies(domain: Il2CppDomain) -> Result<Vec<Assembly>, String> {
        let mut ret: Vec<Assembly> = Vec::new();
        match domain_get_assemblies(domain) {
            Ok(assemblies) => {
                for assembly in assemblies {
                    if let Ok(image) = assembly_get_image(assembly) {
                        let name = image_get_name(image);
                        let file_name = image_get_filename(image);
                        if name.is_err() || file_name.is_err() {
                            continue;
                        }
                        let asm = ret.push_mut(Assembly::new(
                            assembly,
                            name.unwrap(),
                            file_name.unwrap(),
                        ));
                        if let Err(e) = Cache::parse_class(asm, image) {
                            return Err(format!("Failed to parse class {}", e));
                        }
                    }
                }
            }
            Err(e) => return Err(format!("Failed to parse assemblies {}", e)),
        }
        Ok(ret)
    }

    pub fn parse_class(assembly: &mut Assembly, image: Il2CppImage) -> Result<(), String> {
        if let Ok(class_count) = image_get_class_count(image) {
            for i in 0..class_count {
                let p_class = image_get_class(image, i);
                if let Ok(p_class) = p_class {
                    let name = class_get_name(p_class);
                    let namespace = class_get_namespace(p_class);
                    let parent = class_get_parent(p_class);

                    if name.is_err() || namespace.is_err() || parent.is_err() {
                        continue;
                    }

                    let parent_name = if let Ok(parent) = parent {
                        if !parent.is_null() {
                            match class_get_name(parent) {
                                Ok(name) => name,
                                Err(_) => "".to_string(),
                            }
                        } else {
                            "".to_string()
                        }
                    } else {
                        "".to_string()
                    };

                    let class =
                        ClassInner::new(p_class, name.unwrap(), namespace.unwrap(), parent_name);
                    if let Err(e) = Cache::parse_fields(&class) {
                        return Err(format!("Failed to parse fields {}", e));
                    }
                    if let Err(e) = Cache::parse_methods(&class) {
                        return Err(format!("Failed to parse methods {}", e));
                    }
                    assembly.classes.push(class);
                }
            }
        }
        Ok(())
    }

    pub fn parse_fields(class: &Class) -> Result<(), String> {
        let mut iter: *mut u8 = std::ptr::null_mut();
        let mut field: *mut u8;

        loop {
            if let Ok(ff) = class_get_fields(class.address, &mut iter) {
                field = ff;
            } else {
                break;
            }
            if field.is_null() {
                break;
            }

            let name = field_get_name(field);
            if name.is_err() {
                continue;
            }

            let itype = field_get_type(field);
            if itype.is_err() {
                continue;
            }

            let itype = itype.unwrap();
            let type_name = type_get_name(itype);
            if type_name.is_err() {
                continue;
            }
            let type_name = type_name.unwrap();
            let type_ = TypeInner::new(itype, type_name, -1);

            let offset = field_get_offset(field);
            if offset.is_err() {
                continue;
            }
            let offset = offset.unwrap();
            let static_field = if offset <= 0 { true } else { false };

            let name = name.unwrap();
            let weak_cls = Arc::downgrade(class);
            class.fields.write().unwrap().push(FieldInner::new(
                field,
                name,
                type_,
                weak_cls,
                offset,
                static_field,
                std::ptr::null_mut(),
            ));
        }
        Ok(())
    }

    pub fn parse_methods(class: &Class) -> Result<(), String> {
        let mut iter: *mut u8 = std::ptr::null_mut();
        let mut method: *mut u8;

        'parsing: loop {
            if let Ok(mm) = class_get_methods(class.address, &mut iter) {
                method = mm;
            } else {
                break;
            }

            if method.is_null() {
                break;
            }

            let name = method_get_name(method);
            if name.is_err() {
                continue;
            }
            let name = name.unwrap();
            let weak_cls = Arc::downgrade(class);

            let return_type = method_get_return_type(method);
            if return_type.is_err() {
                continue;
            }

            let return_type = return_type.unwrap();
            let return_type_name = type_get_name(return_type);
            if return_type_name.is_err() {
                continue;
            }
            let return_type_name = return_type_name.unwrap();
            let return_type = TypeInner::new(return_type, return_type_name, -1);

            let mut iflag: i32 = 0;
            let flags = method_get_flags(method, &mut iflag);
            if flags.is_err() {
                continue;
            }

            let flags = flags.unwrap();
            let static_function = (flags & 0x10) != 0;
            let ptr_to_func = method as *mut *mut c_void;
            let func_ptr = unsafe { std::ptr::read(ptr_to_func) };

            let arg_count = method_get_param_count(method);
            if arg_count.is_err() {
                continue;
            }
            let arg_count = arg_count.unwrap();
            let args = RwLock::new(Vec::new());
            for i in 0..arg_count {
                let param_name = method_get_param_name(method, i);
                if param_name.is_err() {
                    break 'parsing;
                }
                let param_name = param_name.unwrap();

                let param_type = method_get_param(method, i);
                if param_type.is_err() {
                    break 'parsing;
                }
                let param_type = param_type.unwrap();

                let param_type_name = type_get_name(param_type);
                if param_type_name.is_err() {
                    break 'parsing;
                }
                let param_type_name = param_type_name.unwrap();

                let type_ = TypeInner::new(param_type, param_type_name, -1);
                args.write().unwrap().push(ArgInner::new(param_name, type_));
            }

            class.methods.write().unwrap().push(MethodInner::new(
                method,
                name,
                weak_cls,
                return_type,
                flags,
                static_function,
                func_ptr as *mut u8,
                args,
            ));
        }
        Ok(())
    }

    pub fn new(domain: Il2CppDomain) -> Result<Self, String> {
        match Self::parse_assemblies(domain) {
            Ok(assemblies) => Ok(Self { assemblies }),
            Err(e) => Err(e),
        }
    }
}

impl Debug for Cache {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cache Len: {}\n", self.assemblies.len())?;
        for assembly in &self.assemblies {
            write!(f, "{:?}", assembly)?;
        }
        Ok(())
    }
}
