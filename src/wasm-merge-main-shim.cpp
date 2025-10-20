#include <cstdlib> // _Exit
#include <stdexcept> // runtime_error
#include <iostream>

extern "C" int wasm_merge_main_actual(int argc, const char* argv[]);

// A wrapper for the C++ `main` function that catches exceptions.
//
// This is needed because we have asked the `Fatal` type to throw instead of
// exit, for use as a library. But for use as a `bin` we still need to handle
// those exceptions in a similar way to the "real" `wasm-merge` bin.
//
// Since the bin does not use `cxx` it doesn't get baked-in exception handling,
// so we do that here, and the bin calls this main function.
extern "C" int wasm_merge_main(int argc, const char* argv[]) {
  try {
    return wasm_merge_main_actual(argc, argv);
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    // See comments in `Fatal` about `_Exit` and static destructors.
    _Exit(EXIT_FAILURE);
  }
}
