use std::collections::HashMap;
use libloading::{Library, Symbol};
use self::plugin::ZosPluginVTable;
use std::fs;
use std::ffi::CStr;

pub mod plugin;

type PluginCreator = unsafe extern "C" fn() -> *const ZosPluginVTable;

pub struct Plugin {
    _lib: Library,
    vtable: &'static ZosPluginVTable,
}

impl Plugin {
    fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr((self.vtable.name)()).to_str().unwrap()
        }
    }

    fn execute(&self) {
        unsafe {
            (self.vtable.execute)();
        }
    }
}

pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugins(&mut self, dir: &str) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "so" || ext == "dll" || ext == "dylib" {
                                unsafe {
                                    let lib = Library::new(&path).unwrap();
                                    let creator: Symbol<PluginCreator> = lib.get(b"_zos_plugin_create").unwrap();
                                    let vtable = &*creator();
                                    let plugin = Plugin {
                                        _lib: lib,
                                        vtable,
                                    };
                                    self.plugins.insert(plugin.name().to_string(), plugin);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn execute(&self, command: &str) {
        if let Some(plugin) = self.plugins.get(command) {
            plugin.execute();
        } else {
            println!("Unknown command: {}", command);
        }
    }
}
