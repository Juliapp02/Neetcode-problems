pub fn has_duplicate(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut n_copy = vec![0; nums.len()];
    n_copy = nums.to_vec();
    n_copy.sort();

    let mut previous = n_copy[0];

    for &i in n_copy.iter().skip(1) {
        if i == previous {
            return true;
        } else {
            previous = i;
        }
    }

    false
}

fn main() {
    has_duplicate(Vec::new());
}
