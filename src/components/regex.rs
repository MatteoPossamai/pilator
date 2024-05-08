use crate::components::items::RegexComponent;

#[derive(Debug, Clone)]
pub struct Regex {
    pub components: Vec<RegexComponent>,
}

impl Regex {
    pub fn new(components: Vec<RegexComponent>) -> Self {
        Regex { components }
    }

    pub fn is_match(&self, s: &str) -> bool {
        let mut s = s;
        for component in &self.components {
            match component {
                RegexComponent::Keyword(k) => {
                    if !s.starts_with(k) {
                        return false;
                    }
                    s = &s[k.len()..];
                }
                RegexComponent::Literal(l) => {
                    if !s.starts_with(l) {
                        return false;
                    }
                    s = &s[l.len()..];
                }
                RegexComponent::ZeroOrMore(r) => {
                    while r.is_match(s) {
                        s = &s[r.components.len()..];
                    }
                }, 
                _ => panic!("Not implemented"),
            }
        }
        s.is_empty()
    }
}