use std::process::Command;

fn main() {
    // Run npm install in the app/WhereIsThePower directory
    Command::new("npm")
        .arg("install")
        .current_dir("../app/WhereIsThePower")
        .spawn()
        .expect("Couldn't spawn npm command!")
        .wait()
        .expect("Failed npm install");

    // Build the front-end static files
    Command::new("npm")
        .args(["run", "build"])
        .current_dir("../app/WhereIsThePower")
        .spawn()
        .expect("Couldn't spawn npm command!")
        .wait()
        .expect("Failed npm build");

    // Remove previous build artifacts, if they exist
    if let Err(err) = std::fs::remove_dir_all("./www") {
        eprintln!("Couldn't remove www directory! {err:?}");
    }

    // Copy the front-end static files to our directory
    copy_dir::copy_dir("../app/WhereIsThePower/www", "./www")
        .expect("Couldn't copy www directory!");

    // Rerun this script if the front-end project changes
    println!("cargo:rerun-if-changed=../app/WhereIsThePower/")
}
