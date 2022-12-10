use std::path::Path;
use std::process::Command;

use crate::backend::packages::LaravelPackage;

pub fn create_laravel(path: &String) {
    if Path::new(path).is_dir() {
        panic!("A project with the same name already exists");
    }

    Command::new("composer")
        .args(["create-project", "laravel/laravel", path])
        .output()
        .expect("Error creating new laravel project");

    println!("CREATED: {}", path);
}

pub fn require_package(package: &LaravelPackage, path: &String) {
    match package {
        LaravelPackage::JsonApi => {
            Command::new("composer")
                .args(["require", "laravel-json-api/laravel"])
                .current_dir(path)
                .output()
                .expect("Failed to require JSON:API package");
            Command::new("composer")
                .args(["require", "--dev", "laravel-json-api/testing"])
                .current_dir(path)
                .output()
                .expect("Failed to require JSON:API testing package");
            println!("REQUIRE PACKAGE: JSON:API");

            Command::new("php")
                .args([
                    "artisan",
                    "vendor:publish",
                    format!(r#"--provider="LaravelJsonApi\Laravel\ServiceProvider"#).as_str(), // "--provider=\"LaravelJsonApi\\Laravel\\ServiceProvider\"",
                ])
                .current_dir(path)
                .output()
                .expect("Failed to publish JSON:API configuration");

            println!("PUBLISH CONFIG: JSON:API")
        }
        LaravelPackage::AdminLTE => {}
    }
}
