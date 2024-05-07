pub mod naive_parser;
pub mod llparsers;
pub mod slrparsers;

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

    fn parse(&self, input: &str, config: Option<Self::Config>) -> Result<Vec<String>, String>;

    fn custom_parse(_input: &str, _config: Option<Self::Config>) -> Result<Vec<String>, String> {
        Err("Not implemented".to_string())
    }

    fn tokenize_helper(_input: Vec<char>, _input_idx: usize, _regex: &Regex, _regex_idx: usize, _result: &mut Vec<String>) -> Result<u32, String> {
        Err("This should be private function".to_string())
    }

}