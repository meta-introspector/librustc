//! Universal rustc dynamic loader
//! 
//! Load rustc_driver.so from any version without static linking.
//! Works in CLI tools, compiler plugins, servers, and via C FFI.

pub mod loader;
pub mod ffi;

pub use loader::RustcHandle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_env() {
        if let Ok(path) = std::env::var("RUSTC_DRIVER_SO") {
            let handle = RustcHandle::load(&path);
            assert!(handle.is_ok());
        }
    }
}
