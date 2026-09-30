fn is_valid(s: String) -> bool {
    if !s.chars().count().is_multiple_of(2) {
        return false;
    }

    s.chars().eq(s.chars().rev())
}

fn main() {
    let mut s = String::new();
    s.push_str("([{}])");
    let solution = is_valid(s);
    println!("{}", solution)
}
