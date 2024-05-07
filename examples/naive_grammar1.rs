use pilator::{
    components::items::RegexComponent,
    components::regex::Regex,
    parser::{naive_parser::NaiveParser, Parser},
};

fn main() {
    // Define Regex
    let litteral_1 = RegexComponent::Literal("a".to_string());
    let regex_1 = Regex::new(vec![litteral_1]);

    let mut s = NaiveParser::new();
    s.add_regex(regex_1);
    match s.parse("a", None) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}

// cargo run --emaple naive_grammar1.rs
