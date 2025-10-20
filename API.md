# wasm-merge-sys API Documentation

## Overview

`wasm-merge-sys` provides Rust bindings to Binaryen's `wasm-merge` tool using the 2024 edition of Rust.

## Public API

### Function: `run_wasm_merge`

```rust
pub fn run_wasm_merge(args: &[String]) -> i32
```

Run wasm-merge with the given command-line arguments.

#### Parameters

- `args: &[String]` - A slice of string arguments. The first element should be the program name (typically "wasm-merge"), followed by the actual command-line arguments.

#### Returns

- `i32` - The exit code of wasm-merge:
  - `0` - Success
  - Non-zero - Error occurred

#### Example: Display Help

```rust
use wasm_merge_sys::run_wasm_merge;

let args = vec![
    "wasm-merge".to_string(),
    "--help".to_string(),
];

let exit_code = run_wasm_merge(&args);
```

#### Example: Merge Two Modules

```rust
use wasm_merge_sys::run_wasm_merge;

let args = vec![
    "wasm-merge".to_string(),
    "input1.wasm".to_string(),   // First input file
    "module1".to_string(),        // Name for first module
    "input2.wasm".to_string(),   // Second input file
    "module2".to_string(),        // Name for second module
    "-o".to_string(),             // Output flag
    "merged.wasm".to_string(),    // Output file
];

let exit_code = run_wasm_merge(&args);
if exit_code == 0 {
    println!("Modules merged successfully!");
} else {
    eprintln!("Merge failed with exit code: {}", exit_code);
}
```

#### Example: Merge with Additional Options

```rust
use wasm_merge_sys::run_wasm_merge;

let args = vec![
    "wasm-merge".to_string(),
    "input1.wasm".to_string(),
    "module1".to_string(),
    "input2.wasm".to_string(),
    "module2".to_string(),
    "-o".to_string(),
    "merged.wasm".to_string(),
    "-S".to_string(),             // Emit text instead of binary
    "-g".to_string(),             // Include debug info
];

let exit_code = run_wasm_merge(&args);
```

## Low-Level API (Unsafe)

### Function: `wasm_merge_main` (FFI)

```rust
extern "C" {
    fn wasm_merge_main(argc: c_int, argv: *const *const c_char) -> c_int;
}
```

Direct FFI binding to the C++ `wasm_merge_main` function. This is exposed for advanced use cases but **most users should use the safe `run_wasm_merge` wrapper instead**.

#### Safety

This function is unsafe because:
- It takes raw pointers to C strings
- May call `exit()` or `abort()` on fatal errors
- Assumes `argv` is valid for `argc` elements
- No automatic memory management

## Command-Line Options

The wasm-merge tool supports various options (use `--help` to see all):

- `<input.wasm> <name>` - Input file and its module name (repeat for each module)
- `-o, --output <file>` - Output file
- `-S` - Emit text format instead of binary
- `-g, --debuginfo` - Emit names section and debug info
- `--rename-export-conflicts` - Rename conflicting exports
- `--skip-export-conflicts` - Skip modules with conflicting exports
- `--verbose` - Verbose output

## Building

See the main [README.md](README.md) for build instructions.

## License

MIT OR Apache-2.0
