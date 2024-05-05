pub trait RegexComponent{
    fn match_item(&self, item: String) -> bool;
}

pub struct Literal {
    pub value: String,
}


// impl RegexComponent for Literal {}


// #[derive(Debug)]
// pub struct Keyword {
//     pub value: String,
// }

// impl RegexComponent for Keyword {}
