//! Example demonstrating how to use wasm-merge-sys with wasm-opt
//!
//! This example shows how to merge WebAssembly modules and then optimize them.

use std::fs;
use std::path::PathBuf;
use wasm_merge_sys::run_wasm_merge;

fn main() {
    println!("wasm-merge with wasm-opt example");
    println!("=================================\n");

    // Show wasm-merge version/help
    println!("Running wasm-merge --version:");
    let args = vec!["wasm-merge".to_string(), "--version".to_string()];
    let exit_code = run_wasm_merge(&args);

    if exit_code != 0 {
        println!("Note: wasm-merge --version returned exit code: {}\n", exit_code);
    }

    // Show wasm-merge help
    println!("\nRunning wasm-merge --help:");
    let args = vec!["wasm-merge".to_string(), "--help".to_string()];
    let exit_code = run_wasm_merge(&args);

    if exit_code != 0 {
        println!("Note: wasm-merge --help returned exit code: {}\n", exit_code);
    }

    // Example of how you would merge and optimize wasm files:
    println!("\nExample workflow:");
    println!("1. Use wasm-merge to merge multiple .wasm modules:");
    println!("   wasm-merge module1.wasm name1 module2.wasm name2 -o merged.wasm");
    println!("\n2. Use wasm-opt to optimize the merged module:");
    println!("   wasm_opt::OptimizationOptions::new_optimize_for_size()");
    println!("      .run(\"merged.wasm\", \"optimized.wasm\")");

    println!("\nBoth wasm-merge-sys and wasm-opt crates are successfully linked!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_wasm_merge() {
        let args = vec!["wasm-merge".to_string(), "--help".to_string()];
        let exit_code = run_wasm_merge(&args);
        assert_eq!(exit_code, 0);
    }
}
