pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_so_far = nums[0];
    let mut current = 0;

    for n in nums {
        if current < 0 {
            current = 0;
        }
        current += n;
        if current > max_so_far {
            max_so_far = current;
        }
    }

    max_so_far
}

fn main() {
    println!("{}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    println!("{}", max_sub_array(vec![1]));
    println!("{}", max_sub_array(vec![5, 4, -1, 7, 8]));
    println!("{}", max_sub_array(vec![-2, -1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array(vec![-2, -1]), -1);
        assert_eq!(max_sub_array(vec![-1]), -1);
    }
}
