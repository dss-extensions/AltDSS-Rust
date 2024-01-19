// Copyright 2023 PMeira
// Copyright 2023 DSS-Extensions Contributors
// Copyright 2023 Electric Power Research Institute, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let pwd_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let inc_path = Path::new(&*pwd_dir).join("./dss_capi/include");
    let lib_path = Path::new(&*pwd_dir).join("./dss_capi/lib/linux_x64");
    let profile = env::var("PROFILE").unwrap();

    println!("cargo:rustc-link-arg=-Wl,-rpath={}", lib_path.to_str().unwrap());
    // Select the library binary according to the build profile
    match profile.as_str() {
        "debug" => println!("cargo:rustc-link-lib=dylib=dss_capid"),
        _ => println!("cargo:rustc-link-lib=dylib=dss_capi")
    }
    println!("cargo:rustc-link-search=native={}", lib_path.to_str().unwrap());
    // println!("cargo:rerun-if-changed=src/dss_capi_wrapper.h");

    // https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", inc_path.to_str().unwrap()))
        .header_contents("dss_capi_wrapper.h", "#include \"dss_capi_ctx.h\"")
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");    
}
