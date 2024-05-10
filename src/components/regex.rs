use std::ops::Add;

use crate::components::items::RegexComponent;

#[derive(Debug, Clone)]
pub struct Regex {
    pub components: Vec<RegexComponent>,
}

impl Regex {
    pub fn new(components: Vec<RegexComponent>) -> Self {
        Regex { components }
    }
}

impl Add<Regex> for Regex {
    type Output = Regex;

    fn add(self, other: Regex) -> Regex {
        let mut components = self.components.clone();
        components.extend(other.components);
        Regex { components }
    }
}