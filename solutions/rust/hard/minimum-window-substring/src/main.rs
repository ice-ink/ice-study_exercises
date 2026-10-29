use std::collections::HashMap;

pub fn min_window(s: &str, t: &str) -> String {
    if s.len() < t.len() {
        return String::new();
    }

    // Constraints guarantee ASCII (English letters), so byte indexing is safe.
    let s_bytes = s.as_bytes();
    let mut need: HashMap<u8, usize> = HashMap::new();
    for &b in t.as_bytes() {
        *need.entry(b).or_insert(0) += 1;
    }

    let required = need.len(); // distinct characters we must satisfy
    let mut window: HashMap<u8, usize> = HashMap::new();
    let mut have = 0usize; // distinct characters currently satisfied
    let mut left = 0usize;
    let mut best_left = 0usize;
    let mut best_len = usize::MAX;

    for right in 0..s_bytes.len() {
        let b = s_bytes[right];
        *window.entry(b).or_insert(0) += 1;

        if window.get(&b) == need.get(&b) {
            have += 1;
        }

        // Shrink from the left while the window still satisfies all needs.
        while have == required {
            let len = right - left + 1;
            if len < best_len {
                best_len = len;
                best_left = left;
            }

            let lb = s_bytes[left];
            if let Some(cnt) = window.get_mut(&lb) {
                *cnt -= 1;
            }
            if window.get(&lb) < need.get(&lb) {
                have -= 1;
            }
            left += 1;
        }
    }

    if best_len == usize::MAX {
        String::new()
    } else {
        s[best_left..best_left + best_len].to_string()
    }
}

fn main() {
    println!("{}", min_window("ADOBECODEBANC", "ABC"));
    println!("{}", min_window("a", "a"));
    println!("{}", min_window("a", "aa"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(min_window("a", "a"), "a");
        assert_eq!(min_window("a", "aa"), "");
        assert_eq!(min_window("ab", "b"), "b");
    }
}
