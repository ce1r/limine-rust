fn main() {
    #[cfg(feature = "test")]
    {
        use std::env;
        use std::path::PathBuf;

        println!("cargo::rerun-if-changed=limine.h");

        let bindings = bindgen::Builder::default()
            .header("limine.h")
            .use_core()
            .generate()
            .unwrap();

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings.write_to_file(out_path.join("limine.rs")).unwrap();
    }
}
