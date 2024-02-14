fn main() {
    if std::fs::read("holidays").is_err() {
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
        std::fs::write("holidays", e.finish().unwrap()).unwrap();
    }
}
