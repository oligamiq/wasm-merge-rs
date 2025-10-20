//! Example demonstrating how to use wasm-merge-sys
//!
//! This example shows how to merge WebAssembly modules using the Rust bindings.

use wasm_merge_sys::run_wasm_merge;

fn main() {
    // Example arguments for wasm-merge
    // Format: wasm-merge <input1.wasm> <name1> <input2.wasm> <name2> -o <output.wasm>
    let args = vec![
        "wasm-merge".to_string(),
        "--help".to_string(),
    ];

    let exit_code = run_wasm_merge(&args);
    
    if exit_code == 0 {
        println!("wasm-merge executed successfully");
    } else {
        eprintln!("wasm-merge failed with exit code: {}", exit_code);
        std::process::exit(exit_code);
    }
}
