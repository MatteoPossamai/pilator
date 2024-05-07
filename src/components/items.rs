/// enum RegexComponent 
/// This enum is implemented by all the components that can be part of a regex.
/// It has the method is_nullable that returns a boolean indicating if the component can be nullable or not.
pub enum RegexComponent {
    Literal(String), 
    Keyword(String), 
    ZeroOrMore(String),
    OneOrMore(String),
    ZeroOrOne(String),
    Or(String),
    Group(String),
    Optional(String),    
}

impl RegexComponent {
    /// is_nullable
    /// This method returns a boolean indicating if the component can be nullable or not.
    pub fn is_nullable(&self) -> bool {
        match self {
            RegexComponent::Literal(_) => false,
            RegexComponent::Keyword(_) => false,
            RegexComponent::ZeroOrMore(_) => true,
            RegexComponent::OneOrMore(_) => false,
            RegexComponent::ZeroOrOne(_) => true,
            RegexComponent::Or(_) => false,
            RegexComponent::Group(_) => false,
            RegexComponent::Optional(_) => true,
        }
    }
}