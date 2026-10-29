use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        *freq.entry(num).or_default() += 1;
    }

    // buckets[count] holds every number that appears exactly `count` times.
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
    for (&num, &count) in &freq {
        buckets[count].push(num);
    }

    let mut result: Vec<i32> = Vec::with_capacity(k as usize);
    for i in (0..buckets.len()).rev() {
        for &num in &buckets[i] {
            result.push(num);
            if result.len() == k as usize {
                return result;
            }
        }
    }

    result
}

fn main() {
    println!("{:?}", top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)); // [1, 2]
    println!("{:?}", top_k_frequent(vec![1], 1));                 // [1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
