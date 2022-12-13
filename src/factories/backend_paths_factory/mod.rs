use crate::enums::{BackendFramework, BackendPaths};
use crate::laravel::structs::LaravelProjectPaths;
use crate::nestjs::structs::NestjsProjectPaths;

pub struct BackendPathsFactory {}

impl BackendPathsFactory {
    pub fn new(framework: &BackendFramework) -> BackendPaths {
        match framework {
            BackendFramework::Laravel => BackendPaths::Laravel(LaravelProjectPaths::default()),
            BackendFramework::Nestjs => BackendPaths::Nestjs(NestjsProjectPaths::default()),
        }
    }
}
