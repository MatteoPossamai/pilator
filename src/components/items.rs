use crate::components::regex::Regex;

/// enum RegexComponent
/// This enum is implemented by all the components that can be part of a regex.
/// It has the method is_nullable that returns a boolean indicating if the component can be nullable or not.
#[derive(Debug, Clone)]
pub enum RegexComponent {
    Literal(String),
    Keyword(String),
    Operator(String),
    Identifier(String),
    ZeroOrMore(Regex),
    OneOrMore(Regex),
    ZeroOrOne(Regex),
    Or(Regex, Regex),
    SubRegex(Regex),
}

impl RegexComponent {
    /// is_nullable
    /// This method returns a boolean indicating if the component can be nullable or not.
    pub fn is_nullable(&self) -> bool {
        match self {
            RegexComponent::Literal(_) => false,
            RegexComponent::Keyword(_) => false,
            RegexComponent::Operator(_) => false,
            RegexComponent::Identifier(_) => false,
            RegexComponent::ZeroOrMore(_) => true,
            RegexComponent::OneOrMore(_) => false,
            RegexComponent::ZeroOrOne(_) => true,
            RegexComponent::Or(_, _) => false,
            RegexComponent::SubRegex(regex) => {
                for component in regex.components.iter() {
                    if !component.is_nullable() {
                        return false;
                    }
                }
                true
            },
        }
    }
}
