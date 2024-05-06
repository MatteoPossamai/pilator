use crate::components::items::RegexComponent;

pub struct ZeroOrMore {
    pub value: String, 
}

impl RegexComponent for ZeroOrMore {}

pub struct OneOrMore {
    pub value: String,
}

impl RegexComponent for OneOrMore {}

pub struct ZeroOrOne {
    pub value: String,
}

impl RegexComponent for ZeroOrOne {}

pub struct Optional {
    pub value: String,
}

impl RegexComponent for Optional {}

pub struct Or {
    pub value: String,
}

impl RegexComponent for Or {}

pub struct Group {
    pub value: String,
}

impl RegexComponent for Group {}

pub struct Range {
    pub value: String,
}

impl RegexComponent for Range {}

pub struct Any {
    pub value: String,
}

impl RegexComponent for Any {}

