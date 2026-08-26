use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let solution: Vec<Vec<String>> = vec![];
    let mut char_lists: Vec<HashMap<char, i32>> = vec![];

    for i in 0..strs.len() {
        let iter = strs[i].chars();
        for j in iter {
            *char_lists[i].entry(j).or_insert(0) += 1;
        }
    }

    solution
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
