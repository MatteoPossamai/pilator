use crate::components::items::RegexComponent;
pub struct Regex {
    pub components: Vec<Box<dyn RegexComponent>>,
}

impl Regex {
    pub fn new(components: Vec<Box<dyn RegexComponent>>) -> Self {
        Regex { components }
    }
}