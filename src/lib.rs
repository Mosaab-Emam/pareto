pub mod backend;
pub mod dot;
pub mod enums;
pub mod factories;
pub mod helpers;
pub mod laravel;
pub mod nestjs;
pub mod package_managers;
pub mod traits;
pub mod vue;
pub mod zip_helper;

pub struct Schema {
    pub backend: BackendSchema,
}

pub struct BackendSchema {
    pub models: Vec<Model>,
}

impl From<String> for BackendSchema {
    fn from(s: String) -> Self {
        println!("here is your schema: {}", s);
        Self { models: vec![] }
    }
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
}

impl Model {
    fn new(name: String) -> Model {
        Model { name }
    }
}
