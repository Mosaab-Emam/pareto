pub mod vue {
    pub mod packages {
        pub mod vue_query {
            #[derive(Debug)]
            enum QueryType {
                Query,
                Mutation,
            }

            #[derive(Debug)]
            struct Query {
                name: String,
                url: String,
                query_type: QueryType,
            }

            #[derive(Debug)]
            pub struct ComposableFile {
                filename: String,
                queries: Vec<Query>,
            }

            impl ComposableFile {
                pub fn new(filename: String) -> ComposableFile {
                    ComposableFile {
                        filename,
                        queries: vec![],
                    }
                }

                pub fn query(mut self, name: String, url: Option<String>) -> ComposableFile {
                    let url = match url {
                        Some(url) => url,
                        None => name.clone(),
                    };

                    self.queries.push(Query {
                        name,
                        url,
                        query_type: QueryType::Query,
                    });

                    self
                }
            }
        }
    }
}
