# librustc

Universal dynamic rustc loader - no static linking required.

## Features

- Load `librustc_driver-{hash}.so` from any Rust version
- Call rustc_driver_main to compile files
- No static linking to rustc internals
- Works in CLI tools, compiler plugins, servers
- C FFI for GCC/LLVM plugins
- Nix-friendly

## Usage

### Compile a file
```rust
use librustc::RustcHandle;

fn main() {
    let rustc = RustcHandle::load_from_env().unwrap();
    let result = rustc.compile_file("hello.rs").unwrap();
    println!("Compilation result: {}", result);
}
```

### List symbols
```rust
let symbols = rustc.list_common_symbols();
```

### Custom args
```rust
let result = rustc.compile(&["rustc", "file.rs", "--emit=mir"]).unwrap();
```

## Environment

```bash
export RUSTC_DRIVER_SO=$(find $(rustc --print sysroot)/lib -name "librustc_driver-*.so")
```

## Build

```bash
nix develop
cargo build
cargo run --example simple tests/test.rs
cargo run --example compile tests/test.rs
```
