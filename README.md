# wasm-merge-sys

Rust bindings for Binaryen's `wasm-merge` tool.

This crate provides low-level bindings to the `wasm-merge` C++ implementation from the [Binaryen](https://github.com/WebAssembly/binaryen) project.

## What is wasm-merge?

`wasm-merge` is a WebAssembly module merger that loads multiple wasm files, connects them together by hooking up imports to exports, and emits a single merged module. Unlike `wasm-ld`, this does not have the full semantics of native linkers. Instead, `wasm-merge` does at compile time what you can do with JavaScript at runtime: connect wasm modules together.

The result is a single module that behaves the same as the multiple original modules, but you don't need JavaScript to set up the connections between modules anymore. This allows for better optimization opportunities like DCE and inlining.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wasm-merge-sys = "0.1.0"
```

### Basic Example

```rust
use wasm_merge_sys::run_wasm_merge;

fn main() {
    let args = vec![
        "wasm-merge".to_string(),
        "module1.wasm".to_string(),
        "module1".to_string(),
        "module2.wasm".to_string(),
        "module2".to_string(),
        "-o".to_string(),
        "output.wasm".to_string(),
    ];

    let exit_code = run_wasm_merge(&args);
    
    if exit_code == 0 {
        println!("Modules merged successfully!");
    } else {
        eprintln!("Merge failed with exit code: {}", exit_code);
    }
}
```

## API

### `run_wasm_merge(args: &[String]) -> i32`

Run wasm-merge with the given command-line arguments.

- **Arguments**: A slice of string arguments (should include the program name as args[0])
- **Returns**: The exit code of wasm-merge (0 for success, non-zero for error)

This is a safe wrapper around the C `wasm_merge_main` function that handles string conversion and memory management.

## Building

This crate requires:
- A C++17-compatible compiler
- The Binaryen source code (as a git submodule or in the `binaryen/` directory)

To build:

```bash
# If using as a git submodule
git submodule update --init

# Build the crate
cargo build
```

## Features

- `dwarf` (default): Include DWARF debug information support

## License

MIT OR Apache-2.0

## Reference

This project is modeled after [wasm-opt-rs](https://github.com/brson/wasm-opt-rs) by Brian Anderson.
