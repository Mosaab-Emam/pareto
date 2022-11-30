pub mod dot;
pub mod helpers;
pub mod laravel;
pub mod vue;
pub mod zip_helper;

use helpers::Names;

#[derive(Debug)]
struct Resource {
    names: Names,
}

impl Resource {
    fn new(name: String) -> Resource {
        Resource {
            names: Names::new(&name),
        }
    }
}
