//! Example demonstrating how to use wasm-merge-sys with wasm-opt
//!
//! This example shows how to merge WebAssembly modules and then optimize them.

#[cfg(test)]
mod tests {
    use wasm_merge_sys::run_wasm_merge;

    #[test]
    fn help_wasm_merge() {
        let args = vec!["wasm-merge".to_string(), "--help".to_string()];
        let exit_code = run_wasm_merge(&args);
        assert_eq!(exit_code, 0);
    }
}
