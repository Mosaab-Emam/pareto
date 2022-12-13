use crate::{laravel::structs::LaravelProjectPaths, nestjs::structs::NestjsProjectPaths};

#[derive(Debug)]
pub enum BackendFramework {
    Laravel,
    Nestjs,
}

#[derive(Debug)]
pub enum BackendPaths {
    Laravel(LaravelProjectPaths),
    Nestjs(NestjsProjectPaths),
}
