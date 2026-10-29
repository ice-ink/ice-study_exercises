pub fn is_valid(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                let expected = match ch {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                // pop() returns None on an empty stack, which never matches
                if stack.pop() != Some(expected) {
                    return false;
                }
            }
            _ => {} // constraints say this can't happen
        }
    }

    stack.is_empty()
}

fn main() {
    println!("{}", is_valid("()[]{}")); // true
    println!("{}", is_valid("([)]"));   // false
    println!("{}", is_valid("(("));     // false
    println!("{}", is_valid(""));       // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_valid("()[]{}"));
        assert!(is_valid("{[]}"));
        assert!(is_valid(""));
        assert!(!is_valid("([)]"));
        assert!(!is_valid("(("));
        assert!(!is_valid(")"));
    }
}
