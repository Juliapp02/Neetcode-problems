use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut char_lists: HashMap<String, Vec<String>> = HashMap::new();

    for i in strs {
        let mut chars: Vec<char> = i.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();

        /*
         * With or_default the compiler looks at the type of the value
         * of the HashMap adn calls the default value of that type.
         * Better than or_insert(Vec::new())
         * */
        char_lists.entry(key).or_default().push(i);
    }

    char_lists.into_values().collect()
}

fn main() {
    let strs = vec![
        "act".to_string(),
        "pots".to_string(),
        "tops".to_string(),
        "cat".to_string(),
        "stop".to_string(),
        "hat".to_string(),
    ];
    group_anagrams(strs);
}
