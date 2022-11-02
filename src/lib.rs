pub mod helpers;

pub mod vue {
    pub mod packages {
        pub mod vue_query {
            use crate::helpers::RequestMethod;
            use convert_case::{Case, Casing};

            struct Query {
                name: String,
                url: String,
            }

            #[derive(Debug)]
            struct Mutation {
                name: String,
                url: String,
                http_method: RequestMethod,
            }

            #[derive(Debug)]
            pub struct ComposableFile {
                filename: String,
                queries: Vec<Query>,
                mutations: Vec<Mutation>,
            }

            impl ComposableFile {
                pub fn new(filename: String) -> ComposableFile {
                    ComposableFile {
                        filename,
                        queries: vec![],
                        mutations: vec![],
                    }
                }

                pub fn query(mut self, name: String, url: Option<String>) -> ComposableFile {
                    let url = match url {
                        Some(url) => url,
                        None => name.clone(),
                    };

                    self.queries.push(Query { name, url });

                    self
                }

                pub fn mutation(
                    mut self,
                    mut name: String,
                    url: Option<String>,
                    http_method: RequestMethod,
                ) -> ComposableFile {
                    name = format!("use{}Mutation", name.to_case(Case::Pascal));

                    let url = match url {
                        Some(url) => url,
                        None => self.filename.clone(),
                    };

                    self.mutations.push(Mutation {
                        name,
                        url,
                        http_method,
                    });

                    self
                }
            }
        }
    }
}
