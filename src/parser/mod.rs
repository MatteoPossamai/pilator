use crate::components::regex::Regex;

/// Parser trait:
/// this is the trait that every parser in the library should implement.
/// It contains the following methods:
/// - parse: This is the main method that will be called to parse the input string. It takes the input string
/// and an optional configuration parameter, and returns a Result<Regex, String>. The Regex is the result of the
/// parsing, and the String is an error message in case the parsing fails.
/// - custom_parse: This is an optional method that can be implemented by the parser. It takes the input string
/// and an optional configuration parameter, and returns a Result<Regex, String>. The Regex is the result of the
/// parsing, and the String is an error message in case the parsing fails.
pub trait Parser {
    type Config;

    fn parse(&self, input: &str, config: Option<Self::Config>) -> Result<Regex, String>;

    fn custom_parse(_input: &str, _config: Option<Self::Config>) -> Result<Regex, String> {
        Err("Not implemented".to_string())
    }

}
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

    fn parse(&self, input: &str, _config: Option<Self::Config>) -> Result<Regex, String> {
        println!("Standard parse {}", input);

        for regex in self.regexes.iter() {
            // TODO: check if the regex matches the input string, and if it does, return the splitted string 
            // following the logic of the regex
        }

        Err("Not implemented".to_string())
    }

}

pub struct LL1Parser {}
pub struct SLR1Parser {}

