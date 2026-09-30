use std::{collections::HashMap, vec};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = vec![0; 2];
    let mut hash_nums = HashMap::new();

    for i in nums {
        let num = target - i;
        if hash_nums.contains_key(&num) {
            solution[0] = *hash_nums.get(&num).unwrap();
            solution[1] = i as i32;

            return solution;
        }
        hash_nums.insert(nums[i], i as i32);
    }

    solution
}

fn main() {
    let vec = vec![3, 2, 3];
    let target = 6;
    two_sum(vec, target);
}
