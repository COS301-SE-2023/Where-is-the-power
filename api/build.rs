use std::{
    io::Write,
    process::{Command, Stdio},
};

fn main() -> Result<(), std::io::Error> {
    let area_metadata_yaml = std::fs::read_to_string("./eskom-calendar/area_metadata.yaml")
        .expect("Couldn't read area_metadata.yaml");

    let mut cmd = Command::new("yq")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn yq");

    let mut stdin = cmd.stdin.take().expect("Couldn't grab yq's stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(area_metadata_yaml.as_bytes())
            .expect("Failed to write into yq's stdin");
    });

    let output = cmd
        .wait_with_output()
        .expect("Failed to wait for yq to finish executing");

    std::fs::File::create("area_metadata.json")
        .expect("Couldn't create area_metadata.json")
        .write_all(&output.stdout)
        .expect(format!("Couldn't write json data to file").as_ref());

    println!("cargo:rerun-if-changed=eskom-calendar");

    Ok(())
}
