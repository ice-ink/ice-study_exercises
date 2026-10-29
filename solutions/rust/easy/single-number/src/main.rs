pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for n in nums {
        result ^= n;
    }

    result
}

fn main() {
    println!("{}", single_number(vec![2, 2, 1]));
    println!("{}", single_number(vec![4, 1, 2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(single_number(vec![1]), 1);
    }
}
