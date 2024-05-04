pub trait RegexComponent {} 

pub struct Literal {
    pub value: String,
}

impl RegexComponent for Literal {}


pub struct Keyword {
    pub value: String,
}

impl RegexComponent for Keyword {}
