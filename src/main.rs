use std::fs;

use pareto::dot;
use pareto::enums::BackendFramework;
use pareto::factories::backend_project_factory::BackendProjectFactory;

use dmmf::dmmf_from_schema;

fn main() {
    dot::init();

    let schema = fs::read_to_string("./schemas/schema.pareto").expect("Failed to read schema file");
    let dmmf = dmmf_from_schema(&schema);

    let backend = BackendProjectFactory::new(BackendFramework::Laravel, dmmf);
    println!("{:?}", backend.summarise());

    // let name = String::from("ecommerce");
    // let backend = Backend::new(
    //     name,
    //     vec![BackendFeature::Authentication, BackendFeature::AdminPanel],
    // );
}
