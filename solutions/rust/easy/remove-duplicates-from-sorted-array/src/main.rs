pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut slow = 0;
    for fast in 1..nums.len() {
        if nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }

    (slow + 1) as i32
}

fn main() {
    let mut a = vec![1, 1, 2];
    println!("{}", remove_duplicates(&mut a)); // 2
    let mut b = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}", remove_duplicates(&mut b)); // 5
    let mut c = vec![1];
    println!("{}", remove_duplicates(&mut c)); // 1
    let mut d: Vec<i32> = vec![];
    println!("{}", remove_duplicates(&mut d)); // 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(&nums[..2], &[1, 2][..]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(&nums[..5], &[0, 1, 2, 3, 4][..]);

        let mut nums = vec![1];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(&nums[..1], &[1][..]);

        let mut nums: Vec<i32> = vec![];
        assert_eq!(remove_duplicates(&mut nums), 0);
    }
}
