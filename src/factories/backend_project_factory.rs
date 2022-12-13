use crate::{
    enums::{BackendFramework, BackendPaths},
    factories::{
        backend_paths_factory::BackendPathsFactory, controller_factory::ControllerFactory,
    },
    laravel::structs::laravel_project::LaravelProject,
    nestjs::structs::nestjs_project::NestjsProject,
    traits::backend_project::BackendProject,
};

use dmmf::DataModelMetaFormat;

pub struct BackendProjectFactory {}

impl BackendProjectFactory {
    pub fn new(framework: BackendFramework, dmmf: DataModelMetaFormat) -> Box<dyn BackendProject> {
        let controller_path = match BackendPathsFactory::new(&framework) {
            BackendPaths::Laravel(v) => v.controllers_path,
            BackendPaths::Nestjs(v) => v.resources_path,
        };

        let controller = ControllerFactory::new(&framework, &controller_path);
        println!(
            "listen up, this is my controller: {:?}",
            controller.get_path()
        );
        match framework {
            BackendFramework::Laravel => Box::new(LaravelProject::new()),
            BackendFramework::Nestjs => Box::new(NestjsProject::new()),
        }
    }
}
