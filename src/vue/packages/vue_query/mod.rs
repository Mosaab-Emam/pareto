use crate::helpers::RequestMethod;
use convert_case::{Case, Casing};

#[derive(Debug)]
struct Query {
    name: String,
    url: String,
}
impl Query {
    fn build(&self) -> String {
        format!(
            "export const use{}Query = () => useQuery([\"{}\"], useGet(\"{}\"), {{ initialData: [] }});\n",
            self.name.to_case(Case::Pascal),
            self.name,
            self.url
        )
    }
}

#[derive(Debug)]
struct Mutation {
    name: String,
    url: String,
    http_method: RequestMethod,
}
impl Mutation {
    fn build(&self) -> String {
        let mutate_object = match self.http_method {
            RequestMethod::PATCH => "({id, payload})",
            RequestMethod::DELETE => "id",
            _ => "item",
        };

        let axios_composable = match self.http_method {
            RequestMethod::PATCH => "usePatch",
            RequestMethod::DELETE => "useDelete",
            _ => "usePost",
        };

        let payload = match self.http_method {
            RequestMethod::PATCH => ", payload",
            RequestMethod::DELETE => "",
            _ => ", item",
        };

        let quoted_url = match self.http_method {
            RequestMethod::PATCH => format!("`{}`", self.url),
            RequestMethod::DELETE => format!("`{}`", self.url),
            _ => format!("\"{}\"", self.url),
        };

        format!(
            "export const use{}Mutation = () => useMutation({mutate_object} => {axios_composable}({quoted_url}{payload}));\n",
            self.name.to_case(Case::Pascal),
        )
    }
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
        name: String,
        url: Option<String>,
        http_method: RequestMethod,
    ) -> ComposableFile {
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

    pub fn all(mut self, singular: String) -> ComposableFile {
        self.queries.push(Query {
            name: self.filename.clone(),
            url: self.filename.clone(),
        });

        self.mutations.push(Mutation {
            name: format!("create-{}", singular),
            url: self.filename.clone(),
            http_method: RequestMethod::POST,
        });

        self.mutations.push(Mutation {
            name: format!("update-{}", singular),
            url: format!("{}/${{id}}", self.filename.clone()),
            http_method: RequestMethod::PATCH,
        });

        self.mutations.push(Mutation {
            name: format!("delete-{}", singular),
            url: format!("{}/${{id}}", self.filename.clone()),
            http_method: RequestMethod::DELETE,
        });

        self
    }

    pub fn build_imports(&self) -> String {
        format!("import {{ useQuery, useMutation }} from \"vue-query\";\nimport {{ useGet, usePost, usePatch, useDelete }} from \"../useAxios\";\n\n")
    }

    pub fn build(self) -> String {
        format!(
            "{}{}{}",
            self.build_imports(),
            self.queries.iter().map(|q| q.build()).collect::<String>(),
            self.mutations.iter().map(|m| m.build()).collect::<String>() as String
        )
    }
}
