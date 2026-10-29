pub fn first_uniq_char(s: &str) -> i32 {
    let mut counts = [0i32; 26];

    for b in s.bytes() {
        counts[(b - b'a') as usize] += 1;
    }

    for (i, b) in s.bytes().enumerate() {
        if counts[(b - b'a') as usize] == 1 {
            return i as i32;
        }
    }

    -1
}

fn main() {
    println!("{}", first_uniq_char("leetcode"));
    println!("{}", first_uniq_char("aabb"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(first_uniq_char("leetcode"), 0);
        assert_eq!(first_uniq_char("loveleetcode"), 2);
        assert_eq!(first_uniq_char("aabb"), -1);
        assert_eq!(first_uniq_char(""), -1);
    }
}
