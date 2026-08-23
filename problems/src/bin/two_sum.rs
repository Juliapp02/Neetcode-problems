use std::{io::stdout, vec};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = vec![0; 2];

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            let sum1 = nums[i] + nums[j];
            let sum2 = nums[i] + nums[nums.len() - j - 1];
            println!("Suma 1 -> {i} + {j} = {sum1}");
            // println!("Suma 2 -> {i} + {j} = {sum2}");
            if nums[i] + nums[j] == target {
                solution[0] = i as i32;
                solution[1] = j as i32;
            }
            if nums[i] + nums[nums.len() - j - 1] == target {
                solution[0] = i as i32;
                solution[1] = j as i32;
            }
        }
    }

    println!("SOLUTION -> i: {}, j: {}", solution[0], solution[1]);

    solution
}

fn main() {
    let vec = vec![3, 4, 5, 6];
    let target = 7;
    two_sum(vec, target);
}
