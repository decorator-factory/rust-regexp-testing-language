use super::traits::{Regexp, Match};

/// RegexpSpec represents a single property test specification for a pattern
pub enum RegexpSpec<'a, M: Match<'a>> {
    /// all strings from a vector must get a successfull match
    /// using the pattern
    Detects(Vec<&'a str>),

    /// all strings from a vector must NOT get a successfull
    /// match using the pattern
    DoesNotDetect(Vec<&'a str>),

    /// for each tuple `(haystack, needle)`, the pattern must
    /// find `needle` inside of `haystack`
    InsideFinds(Vec<(&'a str, M)>),

    /// for each case `(input, output)`, the pattern replaces
    /// all occurences of itself in `input`, which must result in `output`
    ReplacesWith {
        replacer: &'a str,
        cases: Vec<(&'a str, &'a str)>,
    },
}

use RegexpSpec::*;

impl<'a, M: Match<'a>> RegexpSpec<'a, M> {
    pub fn is_test_passing<R: Regexp<'a, M>>(&self, target: &R) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::traits::StringPattern;

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

    #[test]
    fn replaces_with_validates_replacement_action_performed_by_pattern() {
        let spec = ReplacesWith {
            replacer: "foo",
            cases: vec![
                ("encapsulation", "enfoosulation"),
                ("capacity", "fooacity"),
                ("arcane", "arcane"),
            ],
        };
        let target = StringPattern("cap");
        assert!(spec.is_test_passing(&target));
    }

    #[test]
    fn replaces_with_fails_if_any_replacement_does_not_match_the_spec() {
        let spec = ReplacesWith {
            replacer: "foo",
            cases: vec![
                ("encapsulation", "enfoosulation"),
                ("capacity", "banana"),
                ("arcane", "arcane"),
            ],
        };
        let target = StringPattern("cap");
        assert!(!spec.is_test_passing(&target));
    }
}
