enum RegexpSpec<'a> {
    Detects(Vec<&'a str>),
    DoesNotDetect(Vec<&'a str>),
    InsideFinds(Vec<(&'a str, &'a str)>),
    ReplacesWith {
        replacer: &'a str,
        cases: Vec<(&'a str, &'a str)>,
    },
}

fn main() {
    println!("Hello, world!");
}
