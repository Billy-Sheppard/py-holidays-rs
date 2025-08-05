fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);
    let path = out_dir.join("holidays");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = std::path::Path::new(&manifest_dir);

    if std::fs::read(&path).is_err() && std::env::var("DOCS_RS").is_err() {
        // install holidays package
        std::process::Command::new("python3")
            .arg("-m")
            .arg("venv")
            .arg("python-env")
            .current_dir(out_dir)
            .output()
            .unwrap();

        let venv = out_dir.join("python-env");

        println!("VENV PATH: {}", venv.display());
        // install holidays package

        std::process::Command::new(venv.join("bin").join("pip"))
            .arg("install")
            .arg("holidays")
            .arg("--require-venv")
            .env("VIRTUAL_ENV", &venv)
            .current_dir(out_dir)
            .output()
            .unwrap();

        // generate objects

        let output = std::process::Command::new(venv.join("bin").join("python"))
            .arg(manifest_dir.join("gen_objects.py"))
            .arg("--require-venv")
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .current_dir(out_dir)
            .env("VIRTUAL_ENV", &venv)
            .output()
            .unwrap();

        if !output.status.success() {
            if !output.stderr.is_empty() {
                panic!(
                    "Failed to generate holidays: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            } else {
                panic!(
                    "Failed to generate holidays, exited with status: {}",
                    output.status
                );
            }
        }


        if serde_json::from_slice::<serde_json::Value>(&output.stdout).is_err() {
            panic!("Generated holidays are not valid JSON")
        }

        let mut e = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
        
        // compress the json
        std::io::Write::write_all(&mut e, &output.stdout).unwrap();

        // flush and finish
        std::fs::write(path, e.finish().unwrap()).unwrap();
    }

    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:rerun-if-env-changed=DOCS_RS");
        println!("cargo:rustc-cfg=docs_rs");
    }
}
