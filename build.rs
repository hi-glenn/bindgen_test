use std::{path::PathBuf, env};

fn main() {
    println!("cargo:rustc-link-lib=secp256k1");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// 其中：rustc-link-lib = [KIND =] NAME用来指定 C 库，传递给 cargo 告知 Rust 编译器 rustc 链接 secp256k1 共享库，
// 可选的 KIND 可以是 static，dylib，默认值是动态库 dylib，有关更多详细信息，请参见 rustc --help。