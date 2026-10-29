use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for n in nums {
        if !seen.insert(n) {
            return true;
        }
    }

    false
}

fn main() {
    println!("{}", contains_duplicate(vec![1, 2, 3, 1]));
    println!("{}", contains_duplicate(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(!contains_duplicate(vec![]));
    }
}
