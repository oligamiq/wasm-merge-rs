//! Native build of `wasm-merge`.
//!
//! This crate builds the C++ code for `wasm-merge` from Binaryen
//! and provides minimal Rust bindings.
//!
//! Most users will probably want higher-level APIs built on top of this.

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    /// The wasm-merge main function.
    ///
    /// This is the C wrapper around the actual wasm-merge implementation
    /// that handles exceptions and provides a C-compatible interface.
    ///
    /// # Arguments
    ///
    /// * `argc` - The number of command-line arguments
    /// * `argv` - Pointer to an array of C strings representing the arguments
    ///
    /// # Returns
    ///
    /// The exit code of the wasm-merge tool (0 for success, non-zero for error)
    ///
    /// # Safety
    ///
    /// This function is unsafe because it:
    /// - Takes raw pointers to C strings
    /// - May call exit() or abort() on fatal errors
    /// - Assumes argv is valid for argc elements
    fn wasm_merge_main(argc: c_int, argv: *const *const c_char) -> c_int;
}

/// Run wasm-merge with the given command-line arguments.
///
/// This is a safe wrapper around the C `wasm_merge_main` function.
///
/// # Arguments
///
/// * `args` - A slice of string arguments (should include the program name as args[0])
///
/// # Returns
///
/// The exit code of wasm-merge (0 for success, non-zero for error)
///
/// # Example
///
/// ```no_run
/// use wasm_merge_sys::run_wasm_merge;
///
/// let args = vec![
///     "wasm-merge".to_string(),
///     "module1.wasm".to_string(),
///     "module1".to_string(),
///     "module2.wasm".to_string(),
///     "module2".to_string(),
///     "-o".to_string(),
///     "output.wasm".to_string(),
/// ];
///
/// let exit_code = run_wasm_merge(&args);
/// assert_eq!(exit_code, 0);
/// ```
pub fn run_wasm_merge(args: &[String]) -> i32 {
    // Convert Rust strings to C strings
    let c_strings: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).expect("CString conversion failed"))
        .collect();

    // Create array of pointers to C strings
    let c_args: Vec<*const c_char> = c_strings.iter().map(|s| s.as_ptr()).collect();

    unsafe { wasm_merge_main(c_args.len() as c_int, c_args.as_ptr()) }
}

/// Just here so that cxx-build becomes willing to manage the set of include
/// directories from this crate for downstream crates to include from.
#[cxx::bridge]
mod dummy {}
