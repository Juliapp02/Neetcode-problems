pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut izqda = vec![1; nums.len()];
    let mut dcha = vec![1; nums.len()];
    let mut acumulado = 1;
    let mut solution = vec![1; nums.len()];

    for i in 0..nums.len() {
        izqda[i] = acumulado;
        acumulado *= nums[i];
    }

    acumulado = 1;

    for i in (0..nums.len()).rev() {
        dcha[i] = acumulado;
        acumulado *= nums[i];
    }

    for i in 0..nums.len() {
        solution[i] = izqda[i] * dcha[i];
    }

    solution
}

fn main() {
    let nums = vec![1, 2, 8];
    product_except_self(nums);
}
