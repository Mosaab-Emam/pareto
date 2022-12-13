use crate::traits::backend_project::BackendProject;

#[derive(Debug)]
pub struct NestjsProject {
    resources: Vec<String>,
}

impl BackendProject for NestjsProject {
    fn resources(&self) -> &Vec<String> {
        &self.resources
    }
}

impl NestjsProject {
    pub fn new() -> Self {
        Self { resources: vec![] }
    }
}
