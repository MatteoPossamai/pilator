use std::vec;

use crate::components::items::RegexComponent;
use crate::components::regex::Regex;
use crate::parser::Parser;

/// NaiveParser:  
/// Simple parser that implements a naive parsing algorithm solely based on brute forcing
/// the input string with all the regular expressions that have been provided.
/// This is a good parser if the language is pretty simple, with low number of regular expressions
/// to be checked, or if the input string is not too long.
///
/// ```rust
/// use pilator::{parser::{naive_parser::NaiveParser, Parser}, components::items::RegexComponent, components::regex::Regex};
/// fn test_naive_parser_litterals() {
///     let litteral_1 = RegexComponent::Literal("a".to_string());
///     let regex_1 = Regex::new(vec![litteral_1.clone(), RegexComponent::Literal(" ".to_string()), litteral_1.clone()]);
///     let mut s = NaiveParser::new();
///     s.add_regex(regex_1);
///     match s.parse("a a", None) {
///         Ok(r) => {
///             assert_eq!(r, vec!["a".to_string(), " ".to_string(), "a".to_string()]);
///         },
///         Err(e) => panic!("Error: {}", e),
///     }
/// }
/// ```
pub struct NaiveParser {
    /// Contains a list of all the Regex that we are going to match against the input string
    regexes: Vec<Regex>,
}

/// Implemented methods:
/// - new: Creates a new instance of NaiveParser
/// - with_regexes: Creates a new instance of NaiveParser with a list of Regex
/// - add_regex: Adds a new Regex to the list of Regexes
/// - get_regexes: Returns a reference to the list of Regexes
/// - remove_regex_with_index: Removes a Regex from the list of Regexes
impl NaiveParser {
    pub fn new() -> NaiveParser {
        NaiveParser { regexes: vec![] }
    }

    pub fn with_regexes(regexes: Vec<Regex>) -> NaiveParser {
        NaiveParser { regexes }
    }

    pub fn add_regex(&mut self, regex: Regex) -> usize {
        self.regexes.push(regex);
        self.regexes.len() - 1
    }

    pub fn get_regexes(&self) -> &Vec<Regex> {
        &self.regexes
    }

    pub fn remove_regex_with_index(&mut self, index: usize) {
        self.regexes.remove(index);
    }


    fn matches(input: String, input_idx: usize, regex: RegexComponent, result: &mut Vec<String>, alarm: &mut bool) -> usize {
        match regex {
            RegexComponent::Literal(value) => {
                if input_idx + value.len() > input.len() {
                    return 0;
                }
                let word = input[input_idx..input_idx + value.len()].to_string();
                if word != value {
                    return 0;
                }
                return value.len();
            }
            RegexComponent::Keyword(value) => {
                if input_idx + value.len() > input.len() {
                    return 0;
                }
                let word = input[input_idx..input_idx + value.len()].to_string();
                if word != value {
                    return 0;
                }
                return value.len();
            }
            RegexComponent::Identifier(value) => {
                if input_idx + value.len() > input.len() {
                    return 0;
                }
                let word = input[input_idx..input_idx + value.len()].to_string();
                if word != value {
                    return 0;
                }
                return value.len();
            }
            RegexComponent::Operator(value) => {
                if input_idx + value.len() > input.len() {
                    return 0;
                }
                let word = input[input_idx..input_idx + value.len()].to_string();
                if word != value {
                    return 0;
                }
                return value.len();
            }
            RegexComponent::ZeroOrMore(value) => {
                let mut temp_idx = input_idx;

                while temp_idx < input.len() {

                    for component in value.components.iter() {
                        let temp = Self::matches(input.clone(), temp_idx, component.clone(), result, alarm);
                        if temp == 0 {
                            return temp_idx - input_idx;
                        }
                        temp_idx += temp;
                    }
                }
                temp_idx - input_idx
            }
            RegexComponent::OneOrMore(value) => {
                let mut temp_idx = input_idx;

                while temp_idx < input.len() {
                    for component in value.components.iter() {
                        let temp = Self::matches(input.clone(), temp_idx, component.clone(), result, alarm);
                        if temp == 0 {
                            return temp_idx - input_idx;
                        }
                        temp_idx += temp;
                    }
                }
                temp_idx - input_idx
            }
            RegexComponent::ZeroOrOne(value) => {
                let mut temp_idx = input_idx;

                for component in value.components.iter() {
                    let temp = Self::matches(input.clone(), temp_idx, component.clone(), result, alarm);
                    if temp == 0 {
                        return temp_idx - input_idx;
                    }
                    temp_idx += temp;
                }
                temp_idx - input_idx
                
            }
            RegexComponent::Or(regex1, regex2) => {
                let temp1 = Self::matches(input.clone(), input_idx, RegexComponent::ZeroOrOne(regex1), result, alarm);
                if temp1 != 0 {
                    return temp1;
                }
                let temp2 = Self::matches(input, input_idx, RegexComponent::ZeroOrOne(regex2), result, alarm);
                temp2
            }
            RegexComponent::SubRegex(regex) => {
                let temp_idx = input_idx;
                let mut temp_res = vec![];
                
                match Self::tokenize_helper(input.clone(), temp_idx, &regex, &mut temp_res, true) {
                    Ok(_) => {
                        let offset: usize = temp_res.iter().map(|x| x.len()).sum();
                        result.append(&mut temp_res);
                        *alarm = true;
                        return offset;
                    },
                    Err(_) => {
                        return 0;
                    },
                }
            }
        }
    }

    fn tokenize_helper(
        input: String,
        input_idx: usize,
        regex: &Regex,
        result: &mut Vec<String>,
        nested: bool
    ) -> Result<u32, String> {
        if input_idx == input.len() {
            // If we reach the end of the input and regex, the regex is a valid match
            return Ok(1);
        } else {
            // Check what the current component is and call the corresponding method
            let mut idx = input_idx;
            let mut regex_idx = 0;
            for component in regex.components.iter() {
                let mut alarm: bool = false;
                let temp = Self::matches(input.clone(), idx, component.clone(), result, &mut alarm);
                regex_idx += 1;
                if temp == 0 {
                    if component.is_nullable() {
                        continue;
                    }
                    return Err("No match".to_string());
                }
                // Check if the component contains a subregex
                if !alarm {
                    result.push(input[idx..idx + temp].to_string());
                }
                idx += temp;
                if idx >= input.len() {
                    break;
                }
            }

            if regex_idx < regex.components.len() {
                for component in regex.components[regex_idx..].iter() {
                    if !component.is_nullable() {
                        return Err("No match".to_string());
                    }
                }
            }

            if idx < input.len() && !nested {
                return Err("No match".to_string());
            }

            Ok(1)
        }
    }
}

/// Parser trait implementation for NaiveParser, using its own Regex
impl Parser for NaiveParser {
    type Config = ();

    fn parse(&self, input: &str, _config: Option<Self::Config>) -> Result<Vec<String>, String> {
        let mut tokens: Vec<String>;

        let input = input.trim().clone();
        for component in self.regexes.iter() {
            tokens = vec![];
            match Self::tokenize_helper(
                input.to_string(),
                0,
                component,
                &mut tokens,
                false
            ) {
                Ok(_) => {
                    return Ok(tokens);
                },
                Err(_) => (),
            }
        }

        Err("Unable to parse from given input".to_string())

    }

    
}

// Unit tests for the created structures
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naive_parser_litterals() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::Literal(" ".to_string()),
            litteral_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("a a", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), " ".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_keyword() {
        let keyword_1 = RegexComponent::Keyword("a".to_string());
        let regex_1 = Regex::new(vec![
            keyword_1.clone(),
            RegexComponent::Literal(" ".to_string()),
            keyword_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("a a", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), " ".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_identifier() {
        let identifier_1 = RegexComponent::Identifier("a".to_string());
        let regex_1 = Regex::new(vec![
            identifier_1.clone(),
            RegexComponent::Literal(" ".to_string()),
            identifier_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("a a", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), " ".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_operator() {
        let operator_1 = RegexComponent::Operator("a".to_string());
        let regex_1 = Regex::new(vec![
            operator_1.clone(),
            RegexComponent::Literal(" ".to_string()),
            operator_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("a a", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), " ".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_set_of_litterals() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            litteral_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aba", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_litterals_and_spaces() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::Literal(" ".to_string()),
            litteral_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("ab a", None) {
            Ok(r) => {
                assert_eq!(
                    r,
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        " ".to_string(),
                        "a".to_string()
                    ]
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_litterals_words_and_keywords() {
        let litteral_1 = RegexComponent::Literal("WORD".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let keyword_1 = RegexComponent::Keyword("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            keyword_1.clone(),
            litteral_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("WORDbcWORD", None) {
            Ok(r) => {
                assert_eq!(
                    r,
                    vec![
                        "WORD".to_string(),
                        "b".to_string(),
                        "c".to_string(),
                        "WORD".to_string()
                    ]
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_keyword_litterals_with_space() {
        let keyword_1 = RegexComponent::Keyword("a".to_string());
        let litteral_1 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            keyword_1.clone(),
            litteral_1.clone(),
            RegexComponent::Literal(" ".to_string()),
            keyword_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("ab a", None) {
            Ok(r) => {
                assert_eq!(
                    r,
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        " ".to_string(),
                        "a".to_string()
                    ]
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_more_with_zero_occurences() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("ab", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_one_zero_occurence() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrOne(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("ab", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_one_one_occurence() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrOne(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aba", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_one_multiple_in_regex() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrOne(Regex::new(vec![litteral_1.clone(), litteral_2.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abab", None) {
            Ok(r) => {
                assert_eq!(
                    r,
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        "ab".to_string(),
                    ]
                );
            }
            Err(e) => panic!("Error: {}", e),
        }

        match s.parse("ab", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_more_zero_occurences() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("ab", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_one_with_following_part() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrOne(Regex::new(vec![litteral_1.clone()])),
            litteral_3.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abc", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_more_with_one_occurence() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            litteral_2.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aba", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_more_with_multiple_occurences() {
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
                assert_eq!(
                    r,
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        "aa".to_string(),
                    ]
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_more_complex() {
        let keyword_1 = RegexComponent::Keyword("IF".to_string());
        let litteral_1 = RegexComponent::Literal("10".to_string());
        let litteral_2 = RegexComponent::Literal("<".to_string());
        let litteral_3 = RegexComponent::Literal("20".to_string());
        let keyword_2 = RegexComponent::Literal("THEN".to_string());
        let keyword_3 = RegexComponent::Literal("PRINT".to_string());
        let white = RegexComponent::Literal(" ".to_string());
        let regex_1 = Regex::new(vec![
            keyword_1.clone(),  // IF
            white.clone(),      // " "
            litteral_1.clone(), // 10
            white.clone(),      // " "
            litteral_2.clone(), // "<"
            white.clone(),      // " "
            litteral_3.clone(), // "20"
            white.clone(),      // " "
            keyword_2.clone(),  // "THEN"
            white.clone(),      // " "
            RegexComponent::ZeroOrMore(Regex::new(vec![keyword_3.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("IF 10 < 20 THEN PRINTPRINTPRINT", None) {
            Ok(r) => {
                assert_eq!(
                    r,
                    vec![
                        "IF".to_string(),
                        " ".to_string(),
                        "10".to_string(),
                        " ".to_string(),
                        "<".to_string(),
                        " ".to_string(),
                        "20".to_string(),
                        " ".to_string(),
                        "THEN".to_string(),
                        " ".to_string(),
                        "PRINTPRINTPRINT".to_string(),
                    ]
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_more_with_following() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_2.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![litteral_1.clone()])),
            litteral_3.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("bac", None) {
            Ok(r) => {
                assert_eq!(r, vec!["b".to_string(), "a".to_string(), "c".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_one_or_more_one_occurence() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::OneOrMore(Regex::new(vec![litteral_1.clone()])),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aab", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "a".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_one_or_more_multiple_occurences() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::OneOrMore(Regex::new(vec![litteral_1.clone()])),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aaab", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "aa".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_one_or_more_only() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let regex_1 = Regex::new(vec![
            RegexComponent::OneOrMore(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aaa", None) {
            Ok(r) => {
                assert_eq!(r, vec!["aaa".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_one_or_more_with_zero_or_more_nested() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::OneOrMore(Regex::new(vec![
                litteral_1.clone(),
                RegexComponent::ZeroOrMore(Regex::new(vec![litteral_2.clone()])),
                litteral_3.clone(),
            ])),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aabcb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "abc".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_zero_or_more_with_one_or_more_nested() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![
                litteral_1.clone(),
                RegexComponent::OneOrMore(Regex::new(vec![litteral_2.clone()])),
                litteral_3.clone(),
            ])),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aabbbbcabcabbbbbcb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "abbbbcabcabbbbbc".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_zero_or_more_with_zero_or_more_nested() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![
                litteral_1.clone(),
                RegexComponent::ZeroOrMore(Regex::new(vec![litteral_2.clone()])),
                litteral_3.clone(),
            ])),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aabbbbcabcabbbbbcb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "abbbbcabcabbbbbc".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_zero_or_more_with_one_or_more_nested_and_zero_or_one() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![
                litteral_1.clone(),
                RegexComponent::OneOrMore(Regex::new(vec![litteral_2.clone()])),
                litteral_3.clone(),
            ])),
            litteral_2.clone(),
            RegexComponent::ZeroOrOne(Regex::new(vec![litteral_1.clone()])),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("aabbbbcabcabbbbbcb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "abbbbcabcabbbbbc".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_or_regex_1() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::Or(
                Regex::new(vec![litteral_2.clone()]),
                Regex::new(vec![litteral_3.clone()]),
            ),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_or_regex_2() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::Or(
                Regex::new(vec![litteral_2.clone()]),
                Regex::new(vec![litteral_3.clone()]),
            ),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("acb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "c".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_subregex() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::SubRegex(Regex::new(vec![
                litteral_2.clone(),
                litteral_3.clone(),
            ]),
            ),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abcb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "c".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_sub_regex_with_zero_or_more() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::SubRegex(Regex::new(vec![
                litteral_2.clone(),
                litteral_3.clone(),
                RegexComponent::ZeroOrMore(Regex::new(vec![litteral_1.clone()])),
            ]),
            ),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abcb", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "c".to_string(), "b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_subregex_with_zero_or_more_with_multiple_occurence() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let litteral_3 = RegexComponent::Literal("c".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::SubRegex(Regex::new(vec![
                litteral_2.clone(),
                litteral_3.clone(),
                RegexComponent::ZeroOrMore(Regex::new(vec![litteral_1.clone()])),
            ]),
            ),
            litteral_2.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abcaaaab", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "c".to_string(), "aaaa".to_string(),"b".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_naive_parser_zero_or_more_with_subregex() {
        let litteral_1 = RegexComponent::Literal("a".to_string());
        let litteral_2 = RegexComponent::Literal("b".to_string());
        let regex_1 = Regex::new(vec![
            litteral_1.clone(),
            RegexComponent::ZeroOrMore(Regex::new(vec![
                RegexComponent::SubRegex(Regex::new(vec![litteral_2.clone()])),
            ])),
            litteral_1.clone(),
        ]);

        let mut s = NaiveParser::new();
        s.add_regex(regex_1);
        match s.parse("abba", None) {
            Ok(r) => {
                assert_eq!(r, vec!["a".to_string(), "b".to_string(), "b".to_string(), "a".to_string()]);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
