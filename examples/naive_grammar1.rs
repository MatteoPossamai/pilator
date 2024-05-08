use pilator::{
    components::items::RegexComponent,
    components::regex::Regex,
    parser::{naive_parser::NaiveParser, Parser},
};

fn main(){
    let keyword_1 = RegexComponent::Keyword("IF".to_string());
    let litteral_1 = RegexComponent::Literal("10".to_string());
    let litteral_2 = RegexComponent::Literal("<".to_string());
    let litteral_3 = RegexComponent::Literal("20".to_string());
    let keyword_2 = RegexComponent::Literal("THEN".to_string());
    let keyword_3 = RegexComponent::Literal("DO".to_string());
    let regex_1 = Regex::new(vec![
        keyword_1.clone(), // IF
        litteral_1.clone(),// 10
        litteral_2.clone(),// "<"
        litteral_3.clone(),// "20"
        keyword_2.clone(), // "THEN"
        RegexComponent::ZeroOrMore(Regex::new(vec![keyword_3.clone()])),
    ]);

    let mut s = NaiveParser::new();
    s.add_regex(regex_1);
    match s.parse("IF10<20THENDO", None) {
        Ok(r) => {
            println!("Result: {:?}", r)
        }
        Err(e) => panic!("Error: {}", e),
    }
}

// cargo run --emaple naive_grammar1.rs
