use crate::il2cpp::classes::assembly::Assembly;

use super::il2cpp::Il2Cpp;
use super::il2cpp_sys::Il2CppApi;
use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::OnceLock;

static IL2CPP_API_LOWLEVEL: OnceLock<Il2CppApi> = OnceLock::new();
static IL2CPP_API_HIGHLEVEL: OnceLock<Il2Cpp<'static>> = OnceLock::new();

// The only api the user should use
pub static IL2CPP_API: std::sync::LazyLock<Arc<Mutex<UnityResolve<'static>>>> =
    LazyLock::new(|| {
        Arc::new(Mutex::new(match UnityResolve::new() {
            Ok(api) => api,
            Err(_) => UnityResolve::new_empty(),
        }))
    });

pub struct UnityResolve<'a> {
    api: Option<&'a Il2Cpp<'a>>,
    p_domain: Option<*mut u8>,
    assemblies: Vec<Assembly>,
}

impl<'a> UnityResolve<'a> {
    pub fn new_empty() -> Self {
        Self {
            api: None,
            p_domain: None,
            assemblies: Vec::new(),
        }
    }

    pub fn new_from_api(api: Option<&'a Il2Cpp<'a>>) -> Self {
        Self {
            api,
            p_domain: Some(std::ptr::null_mut()),
            assemblies: Vec::new(),
        }
    }

    pub fn new() -> Result<Self, String> {
        match UnityResolve::init_il2cpp_api() {
            Ok(_) => {
                if let Some(api) = IL2CPP_API_HIGHLEVEL.get() {
                    Ok(Self::new_from_api(Some(api)))
                } else {
                    Err("Failed to initialize Il2CppApiHighLevel".to_string())
                }
            }

            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn init_il2cpp_api() -> Result<(), String> {
        let mut api = Il2CppApi::new();
        unsafe {
            if let Err(e) = api.initialize("GameAssembly.dll") {
                return Err(format!("Failed to initialize Il2CppApi: {}", e));
            }
        }
        let ret = IL2CPP_API_LOWLEVEL.set(api);
        if ret.is_err() {
            Err("Failed to initialize Il2CppApi".to_string())
        } else {
            unsafe {
                IL2CPP_API_LOWLEVEL
                    .get()
                    .map(|api_lowlevel| IL2CPP_API_HIGHLEVEL.set(Il2Cpp::new(api_lowlevel)));
            }
            if IL2CPP_API_HIGHLEVEL.get().is_none() {
                return Err(format!("Failed to initialize Il2CppApiHighLevel"));
            } else {
                Ok(())
            }
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let api = match self.api {
            Some(a) => a,
            None => return Err("api was none".to_string()),
        };
        if let Some(domain) = api.get_domain() {
            self.p_domain = Some(domain);
            api.thread_attach(domain);
            match self.cache_assemblies() {
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        } else {
            return Err("Could not get domain".to_string());
        }
        Ok(())
    }

    pub fn cache_assemblies(&mut self) -> Result<(), String> {
        let api = match self.api {
            Some(a) => a,
            None => return Err("api was none".to_string()),
        };

        let domain = match self.p_domain {
            Some(a) => a,
            None => return Err("domain was none".to_string()),
        };

        let assemblies = api.domain_get_assemblies(domain);
        for assembly in assemblies {
            if let Some(image) = api.assembly_get_image(assembly) {
                let name = api.image_get_name(image);
                if name.is_none() {
                    continue;
                }
                let filename = api.image_get_filename(image);
                if filename.is_none() {
                    continue;
                }
                let mut asm = Assembly::new(
                    image,
                    String::from(name.unwrap()),
                    String::from(filename.unwrap()),
                );
                self.assemblies.push(asm);
            }
        }
        Ok(())
    }

    pub fn get_api(&self) -> Option<&'a Il2Cpp<'a>> {
        self.api
    }

    pub fn get_domain(&self) -> Option<*mut u8> {
        self.p_domain
    }
}

// NOTE: I dont think this is safe at all, but i need to test it more
unsafe impl<'a> Send for UnityResolve<'a> {}
unsafe impl<'a> Sync for UnityResolve<'a> {}
