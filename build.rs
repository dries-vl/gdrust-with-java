extern crate bindgen;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("./bindings/nativelib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/nativelib.rs")
        .expect("Couldn't write bindings!");

    // CRUCIAL! this tells the compiler to find the dll in the 'bindings' folder
    // -> otherwise generic 1120 linking error (refers to link.exe, but is general error)
    // compiled dll/exe does expect the bindings dll to be in the same folder
    println!("cargo:rustc-link-search=native=bindings");
    println!("cargo:rustc-link-lib=dylib=nativelib");
}
