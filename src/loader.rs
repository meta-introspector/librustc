use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

type RustcMainFn = unsafe extern "C" fn(c_int, *const *const c_char) -> c_int;

#[derive(Debug)]
pub struct RustcHandle {
    library: Library,
}

impl RustcHandle {
    pub fn load(so_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let library = Library::new(so_path)?;
            Ok(Self { library })
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

    pub fn list_common_symbols(&self) -> Vec<String> {
        let symbols = vec![
            "rustc_driver_main",
            "rustc_interface_run_compiler",
            "rustc_session_build_session",
            "rustc_ast_parse_file",
            "rustc_hir_lowering_lower_crate",
        ];

        symbols
            .into_iter()
            .filter(|s| self.find_symbol(s).unwrap_or(false))
            .map(String::from)
            .collect()
    }

    pub fn compile(&self, args: &[&str]) -> Result<i32, Box<dyn std::error::Error>> {
        unsafe {
            let main_fn = self.library.get::<Symbol<RustcMainFn>>(b"rustc_driver_main")?;
            
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
