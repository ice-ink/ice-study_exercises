pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut seen: HashMap<i32, usize> = HashMap::new(); // value -> index

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }

    vec![] // unreachable: exactly one solution is guaranteed
}

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9)); // [0, 1]
    println!("{:?}", two_sum(vec![3, 2, 4], 6));      // [1, 2]
    println!("{:?}", two_sum(vec![3, 3], 6));         // [0, 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
