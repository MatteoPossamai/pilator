use crate::components::regex::Regex;

pub trait Parser {
    type Config;

    fn parse(&self, input: &str, config: Self::Config) -> Result<Regex, String>;

    fn custom_parse(_input: &str, _config: Self::Config) -> Result<Regex, String> {
        Err("Not implemented".to_string())
    }

    fn standard_parse(input: &str, config: Self::Config) -> Result<Regex, String> {
        Err("Not implemented".to_string())
    }
}

pub struct SimpleParser {}

impl Parser for SimpleParser {
    type Config = ();

    fn parse(&self, input: &str, config: Self::Config) -> Result<Regex, String> {
        <SimpleParser as Parser>::standard_parse(input, config)
    }
}
