fn main() {
    if std::fs::read("holidays.zip").is_err() {
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

        // write zip file
        let out = std::fs::File::create("holidays.zip").unwrap();

        let mut zip = zip::ZipWriter::new(out);

        // compress the ron
        zip.start_file("holidays.ron", Default::default()).unwrap();
        std::io::Write::write_all(&mut zip, &py_out).unwrap();

        // flush and finish
        zip.finish().unwrap();

        // delete uncompressed
        std::fs::remove_file("holidays.ron").unwrap();
    }
}
