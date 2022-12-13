use crate::traits::backend_project::BackendProject;

#[derive(Debug)]
pub struct LaravelProject {
    resources: Vec<String>,
}

impl BackendProject for LaravelProject {
    fn resources(&self) -> &Vec<String> {
        &self.resources
    }
}

impl LaravelProject {
    pub fn new() -> Self {
        Self { resources: vec![] }
    }
}
