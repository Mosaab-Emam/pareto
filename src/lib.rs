pub mod backend;
pub mod dot;
pub mod helpers;
pub mod laravel;
pub mod package_managers;
pub mod vue;
pub mod zip_helper;

#[derive(Debug)]
pub struct Resource {
    pub name: String,
}

impl Resource {
    fn new(name: String) -> Resource {
        Resource { name }
    }
}
