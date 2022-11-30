use pareto::dot;
use pareto::laravel;
// use std::fs::{self, copy};

enum Backend {
    Laravel,
}

fn main() {
    dot::init();

    let backend = Backend::Laravel;
    let path = ".";

    // Determine which backend initializer to use
    match backend {
        Backend::Laravel => {
            laravel::Project::new("example-app".to_string())
                .package(laravel::packages::Package::JsonApi)
                .build();
        }
        _ => panic!("This backend is not supported!"),
    }
}
