use librustc::RustcHandle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustc = RustcHandle::load_from_env()?;
    
    println!("✅ Loaded rustc_driver.so");
    
    let symbols = rustc.list_common_symbols();
    println!("Found {} symbols:", symbols.len());
    for sym in symbols {
        println!("  - {}", sym);
    }
    
    // Compile a simple file
    let test_file = std::env::args().nth(1);
    if let Some(file) = test_file {
        println!("\n🔨 Compiling: {}", file);
        let result = rustc.compile_file(&file)?;
        println!("Result: {}", result);
    }
    
    Ok(())
}
