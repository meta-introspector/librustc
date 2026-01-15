use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use goblin::elf::Elf;
use rustc_demangle::demangle;

type RustcMainFn = unsafe extern "C" fn(c_int, *const *const c_char) -> c_int;

#[derive(Debug)]
pub struct RustcHandle {
    library: Library,
    path: String,
}

impl RustcHandle {
    pub fn load(so_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let library = Library::new(so_path)?;
            Ok(Self { 
                library,
                path: so_path.to_string(),
            })
        }
    }

    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let path = std::env::var("RUSTC_DRIVER_SO")
            .or_else(|_| std::env::var("RUSTC_SO"))?;
        Self::load(&path)
    }

    pub fn find_symbol(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        unsafe {
            Ok(self.library.get::<Symbol<unsafe extern "C" fn()>>(name.as_bytes()).is_ok())
        }
    }

    pub fn list_all_symbols(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let buffer = std::fs::read(&self.path)?;
        let elf = Elf::parse(&buffer)?;
        
        let mut symbols = Vec::new();
        for sym in elf.dynsyms.iter() {
            if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                let demangled = demangle(name).to_string();
                symbols.push(demangled);
            }
        }
        Ok(symbols)
    }

    pub fn list_common_symbols(&self) -> Vec<String> {
        let patterns = vec![
            "main",
            "run_compiler",
            "build_session",
        ];

        if let Ok(all_symbols) = self.list_all_symbols() {
            all_symbols
                .into_iter()
                .filter(|s| patterns.iter().any(|p| s.to_lowercase().contains(p)))
                .take(20)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn find_rustc_main(&self) -> Result<String, Box<dyn std::error::Error>> {
        let all_symbols = self.list_all_symbols()?;
        
        // Look for rustc_driver_impl::main
        for sym in &all_symbols {
            if sym.contains("rustc_driver_impl") && sym.contains("::main") {
                // Get the mangled name from ELF
                let buffer = std::fs::read(&self.path)?;
                let elf = Elf::parse(&buffer)?;
                
                for esym in elf.dynsyms.iter() {
                    if let Some(name) = elf.dynstrtab.get_at(esym.st_name) {
                        let demangled = demangle(name).to_string();
                        if demangled == *sym {
                            return Ok(name.to_string());
                        }
                    }
                }
            }
        }
        
        Err("rustc_driver_impl::main not found".into())
    }

    pub fn compile(&self, args: &[&str]) -> Result<i32, Box<dyn std::error::Error>> {
        unsafe {
            // Find the actual rustc_driver_impl::main symbol
            let main_symbol = self.find_rustc_main()?;
            
            let main_fn = self.library.get::<Symbol<RustcMainFn>>(main_symbol.as_bytes())?;
            
            let c_args: Vec<CString> = args.iter()
                .map(|s| CString::new(*s).unwrap())
                .collect();
            
            let c_ptrs: Vec<*const c_char> = c_args.iter()
                .map(|s| s.as_ptr())
                .collect();
            
            let result = main_fn(c_ptrs.len() as c_int, c_ptrs.as_ptr());
            Ok(result)
        }
    }

    pub fn compile_file(&self, file: &str) -> Result<i32, Box<dyn std::error::Error>> {
        self.compile(&["rustc", file])
    }
}
