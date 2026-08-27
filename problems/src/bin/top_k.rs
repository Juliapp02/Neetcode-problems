use std::{cmp::Reverse, collections::HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut fq_nums = HashMap::new();

    for i in nums {
        *fq_nums.entry(i).or_insert(0) += 1;
    }

    let mut sort_map: Vec<(i32, i32)> = fq_nums.into_iter().collect();
    sort_map.sort_unstable_by_key(|pair| Reverse(pair.1));

    let mut solution = vec![];

    for i in 0..k as usize {
        solution.push(sort_map[i].0);
    }

    solution
}

fn main() {
    let nums = vec![1, 2, 2, 3, 3, 3];
    let k = 2;
    top_k_frequent(nums, k);
}
