use pareto::backend::{Backend, BackendFeature};
use pareto::dot;
use pareto::helpers::{Names, Project};
use pareto::laravel::{self, Laravel};
// use std::fs::{self, copy};

enum Facing {
    Frontend,
    Backend,
}

// enum Backend {
//     Laravel,
// }

// #[derive(Debug)]
// pub struct Project {
//     names: Names,
// }

// impl Project {
//     pub fn generate() {}
// }

// pub trait ProjectFactory {
//     fn new(name: &String) -> Project;
// }

// pub struct BackendFactory;

// impl BackendFactory {
//     fn new(name: &String, features: Vec<BackendFeature>) -> Backend {
//         Backend {
//             names: Names::new(name),
//             features,
//         }
//     }
// }

fn main() {
    dot::init();

    let name = String::from("ecommerce");
    let backend = Backend::new(
        name,
        vec![BackendFeature::Authentication, BackendFeature::AdminPanel],
    );

    let laravel_project: Laravel = backend.into();

    println!("my laravel project: {:?}", laravel_project);
    laravel_project.generate();

    // let backend = Backend::Laravel;
    // let path = ".";

    // // Determine which backend initializer to use
    // match backend {
    //     Backend::Laravel => {
    //         laravel::Project::new("example-app".to_string())
    //             .package(laravel::packages::Package::JsonApi)
    //             .build();
    //     }
    //     _ => panic!("This backend is not supported!"),
    // }
}
