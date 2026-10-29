pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut counts = [0i32; 26];

    for (sc, tc) in s.bytes().zip(t.bytes()) {
        counts[(sc - b'a') as usize] += 1;
        counts[(tc - b'a') as usize] -= 1;
    }

    counts.iter().all(|&c| c == 0)
}

fn main() {
    println!("{}", is_anagram("anagram", "nagaram"));
    println!("{}", is_anagram("rat", "car"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_anagram("anagram", "nagaram"));
        assert!(!is_anagram("rat", "car"));
        assert!(is_anagram("", ""));
        assert!(!is_anagram("a", "ab"));
    }
}
