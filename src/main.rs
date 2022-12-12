use std::fs;
use std::path::PathBuf;

use pareto::backend::{Backend, BackendFeature};
use pareto::dot;
use pareto::helpers::Project;
use pareto::laravel::Laravel;
use pareto::Schema;

use dmmf::{self, DataModelMetaFormat};

pub trait ModelTrait {
    fn get_stub_path(self: &Self) -> PathBuf;
    fn generate(self: &Self) -> ();
}

// #[derive(Debug)]
pub struct LaravelModel {}

// impl LaravelModel {
//     pub fn get_stub_path() -> PathBuf {
//         "./stubs/laravel/model.php".into()
//     }

//     pub fn generate(&self) -> () {
//         println!(
//             "migration file generated successfully in {:?}",
//             self.get_stub_path()
//         );
//     }
// }

impl ModelTrait for LaravelModel {
    fn get_stub_path(self: &Self) -> PathBuf {
        "./stubs/laravel/model.php".into()
    }

    fn generate(self: &Self) -> () {
        println!(
            "migration file generated successfully in {:?}",
            self.get_stub_path()
        );
    }
}

impl From<DataModelMetaFormat> for LaravelModel {
    fn from(_dmmf: DataModelMetaFormat) -> Self {
        Self {}
    }
}

// pub struct Laravel {
//     models: Vec<>
// }

fn generate_migration() -> () {
    println!("Generating migration file");
}

fn generate(dmmf: DataModelMetaFormat) -> () {
    let _backend = "laravel";

    dmmf.data_model
        .models
        .into_iter()
        // .for_each(|model| println!("model name: {:?}", model.name));
        .for_each(|model| {
            generate_migration();
        });

    // generate_migration();
}

fn main() {
    dot::init();

    let schema = fs::read_to_string("./schemas/schema.pareto").expect("Failed to read schema file");
    let dmmf = dmmf::dmmf_from_schema(&schema);
    generate(dmmf);
    // let laravel_model: LaravelModel = dmmf.into();

    // println!("Hello world: {:?}", laravel_model.get_stub_path());

    // ---------------------- //

    // let name = String::from("ecommerce");
    // let backend = Backend::new(
    //     name,
    //     vec![BackendFeature::Authentication, BackendFeature::AdminPanel],
    // );

    // let backend: Backend = dmmf.into();

    // let laravel_project: Laravel = backend.into();

    // println!("my laravel project: {:?}", backend);
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
