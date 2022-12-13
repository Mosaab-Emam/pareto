use crate::traits::controller::Controller;

pub mod laravel_project;

#[derive(Debug)]
pub struct LaravelProjectPaths {
    pub migrations_path: String,
    pub models_path: String,
    pub controllers_path: String,
}

impl LaravelProjectPaths {
    pub fn default() -> Self {
        Self {
            migrations_path: "database/migrations".into(),
            models_path: "app/Models".into(),
            controllers_path: "app/Http/Controllers".into(),
        }
    }
}

#[derive(Debug)]
pub struct LaravelController {
    path: String,
}

impl Controller for LaravelController {
    fn get_path(&self) -> &String {
        // &self.contents
        &self.path
    }
}

impl LaravelController {
    pub fn new(path: &String) -> Self {
        // let name: &String = &dmmf.data_model.models[0].name;
        // let names: Names = name.into();
        // let stub = fs::read_to_string("./stubs/laravel/controller")
        //     .expect("Failed to read controller file");
        // let contents = names.replacer(&stub);
        Self {
            path: path.to_owned(),
        }
    }
}
