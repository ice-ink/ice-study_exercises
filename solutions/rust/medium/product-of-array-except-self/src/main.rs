pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1i32; n];

    let mut prefix = 1;
    for i in 0..n {
        result[i] = prefix;
        prefix *= nums[i];
    }

    let mut suffix = 1;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= nums[i];
    }

    result
}

fn main() {
    println!("{:?}", product_except_self(vec![1, 2, 3, 4]));      // [24, 12, 8, 6]
    println!("{:?}", product_except_self(vec![-1, 1, 0, -3, 3])); // [0, 0, 9, 0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
        assert_eq!(product_except_self(vec![0, 0]), vec![0, 0]);
    }
}
