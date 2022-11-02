use std::path::Path;
use std::process::Command;

pub fn create_laravel(path: &String) {
    if Path::new(path).is_dir() {
        panic!("A project with the same name already exists");
    }

    Command::new("composer")
        .arg("create-project")
        .arg("laravel/laravel")
        .arg(path)
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Error creating new laravel project");

    println!("CREATED: {}", path);
}
