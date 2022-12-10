use std::fs;

use pareto::backend::{Backend, BackendFeature};
use pareto::dot;
use pareto::helpers::Project;
use pareto::laravel::Laravel;
use pareto::Schema;

fn main() {
    dot::init();

    let contents = fs::read_to_string("./schemas/blog1.pareto")
        .expect("Should have been able to read the file");

    println!("{}", contents);

    // ---------------------- //

    // let name = String::from("ecommerce");
    // let backend = Backend::new(
    //     name,
    //     vec![BackendFeature::Authentication, BackendFeature::AdminPanel],
    // );

    // let laravel_project: Laravel = backend.into();

    // println!("my laravel project: {:?}", laravel_project);
    // laravel_project.generate();

    // ---------------------- //

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
