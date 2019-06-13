use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;

fn main() {
	let mut bindings_builder = bindgen::Builder::default();

  println!("cargo:rerun-if-env-changed=BINDGEN_DART_SDK_PATH");

	let dartsdk_path = if let Ok(path) = env::var("BINDGEN_DART_SDK_PATH") {
		PathBuf::from(path)
	} else {
    panic!("BINDGEN_DART_SDK_PATH not found in env");
	};
		
  bindings_builder = bindings_builder
    .header(format!("{}/include/dart_api.h", dartsdk_path.display()))
    .header(format!("{}/include/dart_native_api.h", dartsdk_path.display()))
    .header(format!("{}/include/dart_tools_api.h", dartsdk_path.display()));

	let bindings = bindings_builder
    .generate_inline_functions(true)
    .default_enum_style(EnumVariation::Rust)
    .use_core()
    .clang_arg("-std=c++14")
    // required for macOS LLVM 8 to pick up C++ headers:
    .clang_args(&["-x", "c++"])
		.generate()
		.expect("Unable to generate bindings");

	bindings
		.write_to_file("./src/bindings.rs")
		.expect("Couldn't write bindings!");
}
