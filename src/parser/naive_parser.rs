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
}

/// Parser trait implementation for NaiveParser, using its own Regex
impl Parser for NaiveParser {
    type Config = ();

    fn parse(&self, input: &str, _config: Option<Self::Config>) -> Result<Vec<String>, String> {
        let mut valid_strings: u32 = 0;
        let mut tokens: Vec<String> = vec![];

        let input_chars = input.trim().chars().collect::<Vec<char>>();
        for component in self.regexes.iter() {
            tokens = vec![];
            match Self::tokenize_helper(
                input_chars.clone(),
                0,
                component,
                0,
                &mut tokens,
                input.len(),
            ) {
                Ok(count) => valid_strings += count,
                Err(e) => return Err(e),
            }
        }

        if valid_strings == 0 {
            return Err("Unable to parse from given input".to_string());
        } else if valid_strings > 1 {
            return Err("Ambiguous match".to_string());
        }

        Ok(tokens)
    }

    fn tokenize_helper(
        input: Vec<char>,
        input_idx: usize,
        regex: &Regex,
        regex_idx: usize,
        result: &mut Vec<String>,
        end: usize,
    ) -> Result<u32, String> {
        if input_idx == end && regex_idx == regex.components.len() as usize {
            // If we reach the end of the input and regex, the regex is a valid match
            return Ok(1);
        } else if input_idx == end {
            println!("HERE");
            // Check if are all nullable the remaining components
            for i in regex_idx..regex.components.len() {
                if !regex.components[i].is_nullable() {
                    return Ok(0);
                }
            }
            Ok(1)
        } else if regex_idx == regex.components.len() as usize {
            // If we reach the end of the regex, but not the input, the regex is not a valid match
            return Ok(0);
        } else {
            // Check what the current component is and call the corresponding method
            match &regex.components[regex_idx] {
                RegexComponent::Literal(value) => {
                    if input_idx + value.len() > input.len() {
                        return Ok(0);
                    }
                    let current_string = input[input_idx..input_idx + value.len()]
                        .iter()
                        .collect::<String>();
                    if &current_string == value {
                        result.push(value.clone());
                        let temp = Self::tokenize_helper(
                            input,
                            input_idx + value.len(),
                            regex,
                            regex_idx + 1,
                            result,
                            end,
                        );
                        let valid = match temp {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                        return Ok(valid);
                    } else {
                        return Ok(0);
                    }
                }
                RegexComponent::Keyword(value) => {
                    if input_idx + value.len() > input.len() {
                        return Ok(0);
                    }
                    let current_string = input[input_idx..input_idx + value.len()]
                        .iter()
                        .collect::<String>();
                    if &current_string == value {
                        result.push(value.clone());
                        let temp = Self::tokenize_helper(
                            input,
                            input_idx + value.len(),
                            regex,
                            regex_idx + 1,
                            result,
                            end,
                        );
                        let valid = match temp {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                        return Ok(valid);
                    } else {
                        return Ok(0);
                    }
                }
                RegexComponent::ZeroOrMore(value) => {
                    let mut valid_strings: u32 = 0;
                    let mut temp_result: Vec<String> = vec![];
                    let mut temp_idx = input_idx;
                    loop {
                        let mut regex_vector = value.components.clone();
                        regex_vector
                            .append(&mut regex.components[regex_idx + 1..].to_vec().clone());
                        let new_regex = Regex::new(regex_vector);
                        println!("{:?}", new_regex);
                        println!("{:?}", temp_idx + value.components.len());
                        let temp = Self::tokenize_helper(
                            input.clone(),
                            temp_idx,
                            &new_regex,
                            0,
                            &mut temp_result,
                            temp_idx + value.components.len(),
                        );
                        println!("{:?}", temp);
                        result.append(&mut temp_result);
                        println!("{:?}", result);
                        let valid = match temp {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                        println!("{}", valid);
                        if valid == 0 {
                            break;
                        }
                        valid_strings += valid;
                        temp_idx += 1;
                    }
                    if valid_strings == 0 {
                        return Ok(0);
                    }
                    let temp =
                        Self::tokenize_helper(input, temp_idx, regex, regex_idx + 1, result, end);
                    let valid = match temp {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    return Ok(valid);
                }
                RegexComponent::OneOrMore(value) => {
                    let mut valid_strings: u32 = 0;
                    let mut temp_result: Vec<String> = vec![];
                    let mut temp_idx = input_idx;
                    loop {
                        let temp = Self::tokenize_helper(
                            input.clone(),
                            temp_idx,
                            value,
                            0,
                            &mut temp_result,
                            end,
                        );
                        let valid = match temp {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                        if valid == 0 {
                            break;
                        }
                        valid_strings += valid;
                        temp_idx += 1;
                    }
                    if valid_strings == 0 {
                        return Ok(0);
                    }
                    result.append(&mut temp_result);
                    let temp =
                        Self::tokenize_helper(input, temp_idx, regex, regex_idx + 1, result, end);
                    let valid = match temp {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    return Ok(valid);
                }
                RegexComponent::ZeroOrOne(value) => {
                    let mut temp_result: Vec<String> = vec![];
                    let temp = Self::tokenize_helper(
                        input.clone(),
                        input_idx,
                        value,
                        0,
                        &mut temp_result,
                        end,
                    );
                    let valid = match temp {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    if valid == 0 {
                        let temp = Self::tokenize_helper(
                            input,
                            input_idx,
                            regex,
                            regex_idx + 1,
                            result,
                            end,
                        );
                        let valid = match temp {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                        return Ok(valid);
                    }
                    result.append(&mut temp_result);
                    let temp =
                        Self::tokenize_helper(input, input_idx, regex, regex_idx + 1, result, end);
                    let valid = match temp {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    return Ok(valid);
                }
                RegexComponent::Or(value) => {
                    let temp =
                        Self::tokenize_helper(input.clone(), input_idx, value, 0, result, end);
                    let valid = match temp {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    if valid == 0 {
                        let temp = Self::tokenize_helper(
                            input,
                            input_idx,
                            regex,
                            regex_idx + 1,
                            result,
                            end,
                        );
                        let valid = match temp {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                        return Ok(valid);
                    }
                    return Ok(valid);
                }
            };
        }
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
                        "a".to_string(),
                        "a".to_string()
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
            keyword_1.clone(), // IF
            white.clone(),     // " "
            litteral_1.clone(),// 10
            white.clone(),     // " "
            litteral_2.clone(),// "<"
            white.clone(),     // " "
            litteral_3.clone(),// "20"
            white.clone(),     // " "
            keyword_2.clone(), // "THEN"
            white.clone(),     // " "
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
                        "10".to_string(),
                        "<".to_string(),
                        "20".to_string(),
                        "THEN".to_string(),
                        "PRINT".to_string(),
                        "<".to_string()
                    ]
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
