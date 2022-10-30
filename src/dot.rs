use home::home_dir;
use std::fs::create_dir;

pub fn init() {
    let dot_dir = home_dir().unwrap().join(".pareto");

    if !dot_dir.is_dir() {
        println!("~/.pareto does not exists");
        create_dir(&dot_dir.to_str().unwrap()).expect("Failed to create dot directory");
        println!("CREATED: ~/.pareto");
    }
}
