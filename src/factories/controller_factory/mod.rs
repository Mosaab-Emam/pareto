use crate::{
    enums::BackendFramework, laravel::structs::LaravelController,
    nestjs::structs::NestjsController, traits::controller::Controller,
};

pub struct ControllerFactory {}

impl ControllerFactory {
    pub fn new(framework: &BackendFramework, path: &String) -> Box<dyn Controller> {
        match framework {
            BackendFramework::Laravel => Box::new(LaravelController::new(path)),
            BackendFramework::Nestjs => Box::new(NestjsController::new(path)),
        }
    }
}
