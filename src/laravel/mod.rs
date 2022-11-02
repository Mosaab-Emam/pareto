use crate::dot::dotdir;

use regex::Regex;
use std::path::Path;
use std::process::Command;

pub struct Project {
    name: String,
    path: String,
}

impl Project {
    pub fn new(name: String) -> Project {
        Project {
            name: name.clone(),
            path: format!("{}/laravel/{}", dotdir(), name),
        }
    }

    pub fn build(self) {
        if Path::new(&self.path).is_dir() {
            panic!("Already exists brother");
        }

        let output = Command::new("composer")
            .arg("create-project")
            .arg("laravel/laravel")
            .arg(&self.path)
            .output()
            .expect("Error creating new laravel project");

        println!("{}", String::from_utf8(output.stderr).unwrap());
        println!("CREATED: {}", &self.path);
    }
}

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

// fn create_with_composer() {
//     Command::new("composer")
//         .stdout(Stdio::piped())
//         .arg("create-project")
//         .arg("laravel/laravel")
//         .arg(format!("{}/laravel/example-app", dotdir()))
//         .output()
//         .expect("Error creating new laravel project");
// }

fn prepare_for_zip() {
    Command::new("rm")
        .arg("-rf")
        .arg(format!("{}/laravel/example-app/vendor", dotdir()))
        .output()
        .expect("Error deleting vendor directory");
}

pub fn create(path: &str) {
    let _path = Path::new(path);

    let composer_version = get_composer_version(); // Check if composer is installed
    println!("Composer version: {}", composer_version);

    // create_with_composer();
    // println!("CREATED: ~/.pareto/laravel/example-app");

    prepare_for_zip();

    match crate::zip_helper::zip_dir::doit(
        format!("{}/laravel/example-app", dotdir()).as_str(),
        format!("{}/archives/laravel/example-app", dotdir()).as_str(),
        zip::CompressionMethod::Deflated,
    ) {
        Ok(_) => println!("done: {} written to {}", "example-app", "new.zip"),
        Err(e) => println!("Zip Error: {:?}", e),
    }
}
