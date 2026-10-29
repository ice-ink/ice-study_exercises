use std::collections::HashMap;

pub fn length_of_longest_substring(s: &str) -> i32 {
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut left: usize = 0;
    let mut max_length: usize = 0;

    for (right, ch) in s.chars().enumerate() {
        if let Some(&seen_at) = last_seen.get(&ch) {
            if seen_at >= left {
                left = seen_at + 1;
            }
        }
        last_seen.insert(ch, right);
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}

fn main() {
    println!("{}", length_of_longest_substring("abcabcbb")); // 3
    println!("{}", length_of_longest_substring("bbbbb"));    // 1
    println!("{}", length_of_longest_substring("pwwkew"));   // 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(length_of_longest_substring("bbbbb"), 1);
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
        assert_eq!(length_of_longest_substring(""), 0);
        assert_eq!(length_of_longest_substring("aab"), 2);
    }
}
