fn main() {
    let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("holidays");
    if std::fs::read(&path).is_err() && std::env::var("DOCS_RS").is_err() {
        // install holidays package
        std::process::Command::new("pip")
            .arg("install")
            .arg("holidays")
            .output()
            .unwrap();

        // generate objects
        let py_out = std::process::Command::new("python")
            .arg("gen_objects.py")
            .stdout(std::process::Stdio::piped())
            .output()
            .unwrap()
            .stdout;

        let mut e = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
        // compress the ron
        std::io::Write::write_all(&mut e, &py_out).unwrap();

        // flush and finish
        std::fs::write(path, e.finish().unwrap()).unwrap();
    }

    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:rerun-if-env-changed=DOCS_RS");
        println!("cargo:rustc-cfg=docs_rs");
    }
}
