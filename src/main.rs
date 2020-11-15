enum RegexpSpec<'a> {
    Detects(Vec<&'a str>),
    DoesNotDetect(Vec<&'a str>),
    InsideFinds(Vec<(&'a str, &'a str)>),
    ReplacesWith {
        replacer: &'a str,
        cases: Vec<(&'a str, &'a str)>,
    },
}

trait Regexp {
    fn detect(&self, test: &str) -> bool;

    fn find<'t>(&self, haystack: &'t str) -> Option<&'t str>;

    fn replace(&self, input: &str, replacer: &str) -> String;
}

fn main() {
    println!("Hello, world!");
}
