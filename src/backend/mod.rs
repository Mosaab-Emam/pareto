use crate::Resource;

#[derive(Debug)]
pub enum BackendFeature {
    Authentication,
    AdminPanel,
    Database,
    JsonApi,
}

#[derive(Debug)]
pub struct Backend {
    pub name: String,
    pub features: Vec<BackendFeature>,
    pub resources: Vec<Resource>,
}

impl Backend {
    pub fn new(name: String, features: Vec<BackendFeature>) -> Backend {
        let mut resources = vec![];
        for feature in &features {
            match feature {
                BackendFeature::Authentication => resources.push(Resource::new("user".into())),
                BackendFeature::AdminPanel => {}
                BackendFeature::Database => {}
                BackendFeature::JsonApi => {}
            }
        }

        Backend {
            name,
            features,
            resources,
        }
    }
}
