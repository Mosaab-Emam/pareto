pub trait BackendProject {
    fn resources(&self) -> &Vec<String>;
    fn summarise(&self) -> String {
        format!("resources: {:?}", self.resources())
    }
}
