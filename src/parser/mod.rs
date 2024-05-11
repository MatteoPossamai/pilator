pub mod naive_parser;
pub mod llparsers;
pub mod slrparsers;

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
}