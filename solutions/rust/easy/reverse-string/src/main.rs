pub fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0usize;
    let mut right = s.len();

    while left < right {
        right -= 1;
        s.swap(left, right);
        left += 1;
    }
}

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    println!("{:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

        let mut mixed = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut mixed);
        assert_eq!(mixed, vec!['h', 'a', 'n', 'n', 'a', 'H']);

        let mut single = vec!['a'];
        reverse_string(&mut single);
        assert_eq!(single, vec!['a']);

        let mut empty: Vec<char> = Vec::new();
        reverse_string(&mut empty);
        assert!(empty.is_empty());
    }
}
