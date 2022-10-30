use regex::Regex;
use std::path::Path;
use std::process::{Command, Stdio};

fn get_composer_version() -> String {
    let output = Command::new("composer")
        .arg("-v")
        .output()
        .expect("Composer is not installed");

    let output = String::from_utf8(output.stdout).unwrap();

    let result = Regex::new(r"\d+\.\d+\.\d+")
        .unwrap()
        .captures(&output)
        .expect("Unknown composer version");

    return String::from(&result[0]);
}

fn create_with_composer() {
    Command::new("composer")
        .stdout(Stdio::piped())
        .arg("create-project")
        .arg("laravel/laravel")
        .arg("~/example-app")
        .output()
        .expect("Error creating new laravel project");
}

fn prepare_for_zip() {
    Command::new("rm")
        .arg("-rf")
        .arg("example-app/vendor")
        .output()
        .expect("Error deleting vendor directory");
}

pub fn create(path: &str) {
    let _path = Path::new(path);

    let composer_version = get_composer_version(); // Check if composer is installed
    println!("Composer version: {}", composer_version);

    create_with_composer();
    println!("CREATED: ~/.pareto/laravel/example-app");

    prepare_for_zip();

    match crate::zip_helper::zip_dir::doit(
        "~/.pareto/laravel/example-app",
        "~/.pareto/archives/example-app",
        zip::CompressionMethod::Deflated,
    ) {
        Ok(_) => println!("done: {} written to {}", "example-app", "new.zip"),
        Err(e) => println!("Zip Error: {:?}", e),
    }
}
