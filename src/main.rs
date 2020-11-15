pub enum RegexpSpec<'a> {
    Detects(Vec<&'a str>),
    DoesNotDetect(Vec<&'a str>),
    InsideFinds(Vec<(&'a str, Option<&'a str>)>),
    ReplacesWith {
        replacer: &'a str,
        cases: Vec<(&'a str, &'a str)>,
    },
}

use RegexpSpec::*;

pub trait Regexp {
    fn detect(&self, test: &str) -> bool;

    fn find<'t>(&self, haystack: &'t str) -> Option<&'t str>;

    fn replace(&self, input: &str, replacer: &str) -> String;
}

impl<'a> RegexpSpec<'a> {
    pub fn is_test_passing<R: Regexp>(&self, target: &R) -> bool {
        match self {
            Detects(matches) => matches.iter().all(|test| target.detect(test)),

            DoesNotDetect(matches) => !matches.iter().any(|test| target.detect(test)),

            InsideFinds(pairs) => pairs
                .iter()
                .all(|(haystack, needle)| target.find(haystack) == *needle),

            ReplacesWith { replacer, cases } => cases
                .iter()
                .all(|(input, output)| target.replace(input, replacer) == *output),
        }
    }
}

pub struct StringPattern<'a>(&'a str);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_finds_pattern_in_a_list_of_words() {
        let spec = Detects(vec!["banana", "abandon", "kanban"]);
        let target = StringPattern("ban");
        assert!(spec.is_test_passing(&target));
    }

    #[test]
    fn detects_fails_if_any_example_does_not_match() {
        let spec = Detects(vec!["banana", "orange", "abandon", "kanban"]);
        let target = StringPattern("ban");
        assert!(!spec.is_test_passing(&target));
    }

    #[test]
    fn does_not_detect_ensures_none_of_the_strings_pass() {
        let spec = DoesNotDetect(vec!["bread", "fish", "knife"]);
        let target = StringPattern("ban");
        assert!(spec.is_test_passing(&target));
    }

    #[test]
    fn does_not_detect_fails_if_any_example_matches() {
        let spec = DoesNotDetect(vec!["bread", "banana", "fish", "knife"]);
        let target = StringPattern("ban");
        assert!(!spec.is_test_passing(&target));
    }

    #[test]
    fn inside_finds_validates_group_extraction_performed_by_pattern() {
        let spec = InsideFinds(vec![
            ("kanban", Some("ban")),
            ("banana", Some("ban")),
            ("abandon", Some("ban")),
            ("branding", None),
        ]);
        let target = StringPattern("ban");
        assert!(spec.is_test_passing(&target));
    }

    #[test]
    fn inside_finds_fails_if_wrong_string_is_extracted_in_any_case() {
        let spec = InsideFinds(vec![
            ("kanban", Some("kan")),
            ("banana", Some("ban")),
            ("abandon", Some("ban")),
            ("branding", None),
        ]);
        let target = StringPattern("ban");
        assert!(!spec.is_test_passing(&target));
    }

    #[test]
    fn inside_finds_fails_if_extraction_fails_unexpectedly() {
        let spec = InsideFinds(vec![
            ("kanban", Some("ban")),
            ("banana", None),
            ("abandon", Some("ban")),
            ("branding", None),
        ]);
        let target = StringPattern("ban");
        assert!(!spec.is_test_passing(&target));
    }

    #[test]
    fn inside_finds_fails_if_extraction_succeeds_unexpectedly() {
        let spec = InsideFinds(vec![
            ("kanban", Some("ban")),
            ("banana", Some("ban")),
            ("abandon", Some("ban")),
            ("branding", Some("ban")),
        ]);
        let target = StringPattern("ban");
        assert!(!spec.is_test_passing(&target));
    }
}

fn main() {
    println!("Hello, world!");
}
