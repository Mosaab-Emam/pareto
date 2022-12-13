use crate::backend::{Backend, BackendFeature};
use crate::dot::dotdir;
use crate::helpers::Project;
use crate::package_managers::composer;
use crate::Model;

use crate::backend::packages::LaravelPackage;
use regex::Regex;
use std::path::Path;
use std::process::Command;

pub mod structs;

// pub mod packages;

#[derive(Debug)]
pub struct Laravel {
    name: String,
    features: Vec<BackendFeature>,
    resources: Vec<Model>,
}

impl Project for Laravel {
    fn get_path(self: &Self) -> String {
        format!("{}/laravel/{}", dotdir(), self.name)
    }

    fn generate(&self) {
        println!(
            "Generating Laravel project generated in: {}",
            self.get_path()
        );
    }
}

impl From<Backend> for Laravel {
    fn from(backend: Backend) -> Self {
        Self {
            name: backend.name,
            features: backend.features,
            resources: backend.resources,
        }
    }
}

// ---------------------

#[derive(Debug)]
pub struct LaravelProject {
    name: String,
    path: String,
    packages: Vec<LaravelPackage>,
}

impl LaravelProject {
    pub fn new(name: String) -> LaravelProject {
        let p = LaravelProject {
            name: name.clone(),
            path: format!("{}/laravel/{}", dotdir(), name),
            packages: vec![],
        };
        println!("project created {:?}", p);
        panic!("enough");
        p
    }

    pub fn package(mut self, package: LaravelPackage) -> LaravelProject {
        self.packages.push(package);
        self
    }

    pub fn build(self) {
        composer::create_laravel(&self.path);
        self.packages
            .iter()
            .for_each(|package| composer::require_package(package, &self.path));
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
