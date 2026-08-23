use std::collections::HashMap;
use std::iter::zip;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut char_s: HashMap<char, u16> = HashMap::new();
    let mut char_t: HashMap<char, u16> = HashMap::new();

    if s.len() != t.len() {
        return false;
    }

    let mut iter = zip(s.chars(), t.chars());

    for c in iter {
        *char_s.entry(c.0).or_insert(0) += 1;
        *char_t.entry(c.1).or_insert(0) += 1;
    }

    if char_s == char_t {
        return true;
    }

    false
}

fn main() {
    is_anagram(String::new(), String::new());
}
