use pilator::{
    components::{items::RegexComponent, regex::Regex},
    parser::{naive_parser::NaiveParser, Parser},
};

fn main(){
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abaa", None) {
            Ok(r) => {
                println!("Result: {:?}", r);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

// cargo run --emaple naive_grammar1.rs
