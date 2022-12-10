use convert_case::{Case, Casing};
use pluralize_rs::to_plural;

pub trait Project {
    fn get_path(&self) -> String;
    fn generate(&self) {}
}

#[derive(Debug)]
pub struct Names {
    pub singular_pascal: String,
    pub singular_camel: String,
    pub singular_kebab: String,
    pub singular_snake: String,
    pub plural_pascal: String,
    pub plural_camel: String,
    pub plural_kebab: String,
    pub plural_snake: String,
}

impl Names {
    pub fn new(singular: &String) -> Names {
        let plural = to_plural(singular);
        Names {
            singular_pascal: singular.to_case(Case::Pascal),
            singular_camel: singular.to_case(Case::Camel),
            singular_kebab: singular.to_case(Case::Kebab),
            singular_snake: singular.to_case(Case::Snake),
            plural_pascal: plural.to_case(Case::Pascal),
            plural_camel: plural.to_case(Case::Camel),
            plural_kebab: plural.to_case(Case::Kebab),
            plural_snake: plural.to_case(Case::Snake),
        }
    }

    pub fn replacer(&self, contents: &String) -> String {
        contents
            .replace("$$singular_pascal$$", &self.singular_pascal)
            .replace("$$singular_camel$$", &self.singular_camel)
            .replace("$$singular_kebab$$", &self.singular_kebab)
            .replace("$$singular_snake$$", &self.singular_snake)
            .replace("$$plural_pascal$$", &self.plural_pascal)
            .replace("$$plural_camel$$", &self.plural_camel)
            .replace("$$plural_kebab$$", &self.plural_kebab)
            .replace("$$plural_snake$$", &self.plural_snake)
    }
}

#[derive(Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}
