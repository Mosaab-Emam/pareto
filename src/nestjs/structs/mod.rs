use crate::traits::controller::Controller;

pub mod nestjs_project;

#[derive(Debug)]
pub struct NestjsProjectPaths {
    pub resources_path: String,
}

impl NestjsProjectPaths {
    pub fn default() -> Self {
        Self {
            resources_path: "src".into(),
        }
    }
}

#[derive(Debug)]
pub struct NestjsController {
    path: String,
}

impl Controller for NestjsController {
    fn get_path(&self) -> &String {
        // &self.contents
        &self.path
    }
}

impl NestjsController {
    pub fn new(path: &String) -> Self {
        // let name: &String = &dmmf.data_model.models[0].name;
        // let names: Names = name.into();
        // let stub = fs::read_to_string("./stubs/nestjs/controller")
        //     .expect("Failed to read controller file");
        // let contents = names.replacer(&stub);
        Self {
            path: path.to_owned(),
        }
    }
}
