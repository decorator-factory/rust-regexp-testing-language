enum RegexpSpec<'a> {
    Detects(Vec<&'a str>),
    DoesNotDetect(Vec<&'a str>),
    InsideFinds(Vec<(&'a str, &'a str)>),
    ReplacesWith {
        replacer: &'a str,
        cases: Vec<(&'a str, &'a str)>,
    },
}

use RegexpSpec::*;

trait Regexp {
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
                .all(|(haystack, needle)| target.find(haystack) == Some(needle)),

            ReplacesWith { replacer, cases } => cases
                .iter()
                .all(|(input, output)| target.replace(input, replacer) == *output),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
