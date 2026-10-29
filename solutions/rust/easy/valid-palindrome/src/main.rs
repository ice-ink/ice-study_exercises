pub fn is_palindrome(s: &str) -> bool {
    // Input is ASCII only, so we can work with raw bytes (u8) safely.
    let bytes = s.as_bytes();

    // Guard: for an empty string, len() - 1 would underflow (usize can't be negative).
    if bytes.is_empty() {
        return true;
    }

    let mut left: usize = 0;
    let mut right: usize = bytes.len() - 1;

    while left < right {
        if !bytes[left].is_ascii_alphanumeric() {
            left += 1;
            continue;
        }
        if !bytes[right].is_ascii_alphanumeric() {
            right -= 1;
            continue;
        }
        if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

fn main() {
    println!("{}", is_palindrome("A man, a plan, a canal: Panama")); // true
    println!("{}", is_palindrome("hello"));                          // false
    println!("{}", is_palindrome(" "));                              // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome(" "));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("Ab"));
    }
}
