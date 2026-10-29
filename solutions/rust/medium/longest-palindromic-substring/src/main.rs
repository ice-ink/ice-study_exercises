pub fn longest_palindrome(s: &str) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len() as i32;

    let mut start: i32 = 0;
    let mut max_len: i32 = 0;

    for i in 0..n {
        let (s1, l1) = expand_around_center(bytes, i, i);
        let (s2, l2) = expand_around_center(bytes, i, i + 1);

        if l1 > max_len {
            max_len = l1;
            start = s1;
        }
        if l2 > max_len {
            max_len = l2;
            start = s2;
        }
    }

    s[start as usize..(start + max_len) as usize].to_string()
}

fn expand_around_center(bytes: &[u8], mut left: i32, mut right: i32) -> (i32, i32) {
    let n = bytes.len() as i32;
    while left >= 0 && right < n && bytes[left as usize] == bytes[right as usize] {
        left -= 1;
        right += 1;
    }
    (left + 1, right - left - 1)
}

fn main() {
    println!("{}", longest_palindrome("babad"));   // bab
    println!("{}", longest_palindrome("cbbd"));    // bb
    println!("{}", longest_palindrome("racecar")); // racecar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(longest_palindrome("babad"), "bab");
        assert_eq!(longest_palindrome("cbbd"), "bb");
        assert_eq!(longest_palindrome("racecar"), "racecar");
        assert_eq!(longest_palindrome("a"), "a");
        assert_eq!(longest_palindrome(""), "");
    }
}
