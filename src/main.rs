mod laravel {
    use regex::Regex;
    use std::path::Path;
    use std::process::{Command, Stdio};

    fn composer_version() -> String {
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

    pub fn create(path: &str) {
        let path = Path::new(path);

        // Check if composer is installed
        composer_version();

        Command::new("composer")
            .stdout(Stdio::piped())
            .arg("create-project")
            .arg("laravel/laravel")
            .arg("example-app")
            .output()
            .expect("Error creating new laravel project");
    }
}

enum Backend {
    Laravel,
}

fn main() {
    let backend = Backend::Laravel;
    let path = ".";

    // Determine which backend initializer to use
    match backend {
        Backend::Laravel => laravel::create(&path),
        _ => panic!("This backend is not supported!"),
    }

    println!("Hello, world!");
}
