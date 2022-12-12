use dmmf::DataModelMetaFormat;

use crate::Model;

pub mod packages;

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
    pub dmmf: DataModelMetaFormat,
    pub features: Vec<BackendFeature>,
    pub resources: Vec<Model>,
}

impl Backend {
    // pub fn new(name: String, features: Vec<BackendFeature>) -> Backend {
    //     let mut resources = vec![];
    //     for feature in &features {
    //         match feature {
    //             BackendFeature::Authentication => resources.push(Model::new("user".into())),
    //             BackendFeature::AdminPanel => {}
    //             BackendFeature::Database => {}
    //             BackendFeature::JsonApi => {}
    //         }
    //     }

    //     Backend {
    //         name,
    //         features,
    //         resources,
    //     }
    // }
}

// impl From<DataModelMetaFormat> for Backend {
//     fn from(dmmf: DataModelMetaFormat) -> Self {
//         Self {
//             name: "default name".into(),
//             dmmf,
//             features: vec![],
//             resources: vec![],
//         }
//     }
// }
