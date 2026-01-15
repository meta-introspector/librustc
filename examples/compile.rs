use librustc::RustcHandle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file.rs> [rustc args...]", args[0]);
        std::process::exit(1);
    }
    
    let rustc = RustcHandle::load_from_env()?;
    
    // Pass all args to rustc
    let rustc_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    let result = rustc.compile(&rustc_args)?;
    
    std::process::exit(result);
}
