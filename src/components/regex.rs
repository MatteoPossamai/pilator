
use crate::components::items::RegexComponent;
pub struct Regex {
    pub components: Vec<Box<dyn RegexComponent>>,
}