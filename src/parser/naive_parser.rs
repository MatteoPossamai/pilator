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
/// use pilator::parser::*;
///
/// fn main() {
///     let s = NaiveParser::new();
///     match s.parse("a", vec![]) {
///         Ok(_) => println!("Success"),
///         Err(e) => println!("Error in parsing: {}", e),
///     }
///}
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
        // TODO: Implement the logic that tokenizes the string based on the regex
        let mut valid_strings: u32 = 0;
        let mut tokens: Vec<String> = Vec::new();

        let input_chars = input.chars().collect::<Vec<char>>();
        for component in self.regexes.iter() {
            match Self::tokenize_helper(input_chars.clone(), 0, component, 0, &mut tokens) {
                Ok(count) => valid_strings += count,
                Err(e) => return Err(e),
            }
        }

        if valid_strings == 0 {
            return Err("No valid strings found".to_string());
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
    ) -> Result<u32, String> {
        if input_idx == input.len() as usize && regex_idx == regex.components.len() as usize {
            // If we reach the end of the input and regex, the regex is a valid match
            return Ok(1);
        } else if input_idx == input.len() as usize {
            // Check if are all nullable the remaining components
            for i in regex_idx..regex.components.len() {
                if !regex.components[i].is_nullable() {
                    return Ok(0);
                }
            }
            return Ok(1);
        } else if regex_idx == regex.components.len() as usize {
            // If we reach the end of the regex, but not the input, the regex is not a valid match
            return Ok(0);
        } else if input[input_idx] == ' ' {
            // If we find space, we finished the RegexComponent. Check if we completed the regex component, and add the
            // token, otherwise this is not a match. Add the space just if there is not another space in the result
            result.push(" ".to_string());
            return Self::tokenize_helper(input, input_idx + 1, regex, regex_idx + 1, result);
        } else {
            // Check what the current component is and call the corresponding method
            let res = match &regex.components[regex_idx] {
                // TypeId::of::<Literal>() => {
                //     Err("Debug".to_string())
                // }
                RegexComponent::Literal(value) => {
                    // TODO: implement the logic for Literal
                    println!("Literal: {}", value);
                    Err("Not implemented".to_string())
                },
                _ => Err("Not implemented".to_string()),
            };

            return res;
        }
    }
}

pub struct LL1Parser {}
pub struct SLR1Parser {}
