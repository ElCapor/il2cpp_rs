use crate::il2cpp::{
    assembly_get_image, class_get_name, class_get_namespace, class_get_parent,
    classes::{assembly::Assembly, class::Class},
    domain_get_assemblies,
    il2cpp_sys::c_types::{Il2CppDomain, Il2CppImage},
    image_get_class, image_get_class_count, image_get_filename, image_get_name,
};

use std::fmt::{Debug, Formatter};

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
                        match class_get_name(parent) {
                            Ok(name) => name,
                            Err(_) => String::new(),
                        }
                    } else {
                        String::new()
                    };
                    let class = Class::new(p_class, name.unwrap(), namespace.unwrap(), parent_name);
                    assembly.classes.push(class);
                }
            }
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
