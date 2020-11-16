use std::collections::HashMap;

pub trait Regexp<'a, M: Match<'a>> {
    fn detect(&self, test: &str) -> bool;

    fn find(&self, haystack: &'a str) -> M;

    fn replace(&self, input: &str, replacer: &str) -> String;
}

/// StringPattern is a basic pattern type that represents
/// matching against a plain string
pub struct StringPattern<'a>(pub &'a str);

impl<'a, 'b> Regexp<'a, Option<&'a str>> for StringPattern<'b> {
    fn detect(&self, test: &str) -> bool {
        test.contains(&self.0)
    }

    fn find(&self, haystack: &'a str) -> Option<&'a str> {
        haystack
            .find(&self.0)
            .map(|index| &haystack[index..index + self.0.len()])
    }

    fn replace(&self, input: &str, replacer: &str) -> String {
        input.replace(&self.0, replacer)
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum Index {
    Int(u32),
    Named(String),
}

pub trait Match<'a>: Eq {
    fn is_successful(&self) -> bool;

    fn groups(&self) -> HashMap<Index, Option<&'a str>>;
}

impl<'a> Match<'a> for Option<&'a str> {
    fn is_successful(&self) -> bool {
        self.is_some()
    }

    fn groups(&self) -> HashMap<Index, Option<&'a str>> {
        let mut map = HashMap::new();
        map.insert(Index::Int(0), *self);
        map
    }
}
