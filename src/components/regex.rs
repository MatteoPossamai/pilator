use crate::components::items::RegexComponent;
pub struct Regex {
    pub components: Vec<RegexComponent>,
}

impl Regex {
    pub fn new(components: Vec<RegexComponent>) -> Self {
        Regex { components }
    }
}