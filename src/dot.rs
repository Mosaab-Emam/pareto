use home::home_dir;
use std::fs::create_dir;

pub fn dotdir() -> String {
    format!("{}/.pareto", home_dir().unwrap().display())
}

pub fn init() {
    let dot_dir = home_dir().unwrap().join(".pareto");

    if !dot_dir.is_dir() {
        println!("~/.pareto does not exists");
        create_dir(&dot_dir.to_str().unwrap()).expect("Failed to create dot directory");
        println!("CREATED: ~/.pareto");

        create_dir(&dot_dir.join("archives").join("laravel").to_str().unwrap())
            .expect("Failed to create archives/laravel directory");
    }

    if !dot_dir.join("archives").is_dir() {
        create_dir(&dot_dir.join("archives").to_str().unwrap())
            .expect("Failed to create archives directory");
        println!("CREATED: ~/.pareto/archives");
    }

    if !dot_dir.join("archives").join("laravel").is_dir() {
        create_dir(&dot_dir.join("archives").join("laravel").to_str().unwrap())
            .expect("Failed to create archives directory");
        println!("CREATED: ~/.pareto/archives/laravel");
    }
}
