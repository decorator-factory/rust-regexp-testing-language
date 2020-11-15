pub trait Regexp {
    fn detect(&self, test: &str) -> bool;

    fn find<'t>(&self, haystack: &'t str) -> Option<&'t str>;

    fn replace(&self, input: &str, replacer: &str) -> String;
}

/// StringPattern is a basic pattern type that represents
/// matching against a plain string
pub struct StringPattern<'a>(pub &'a str);

impl<'a> Regexp for StringPattern<'a> {
    fn detect(&self, test: &str) -> bool {
        test.contains(&self.0)
    }

    fn find<'t>(&self, haystack: &'t str) -> Option<&'t str> {
        haystack
            .find(&self.0)
            .map(|index| &haystack[index..index + self.0.len()])
    }

    fn replace(&self, input: &str, replacer: &str) -> String {
        input.replace(&self.0, replacer)
    }
}
