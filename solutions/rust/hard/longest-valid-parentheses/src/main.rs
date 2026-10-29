pub fn longest_valid_parentheses(s: &str) -> i32 {
    let mut stack: Vec<i32> = vec![-1];
    let mut max_len = 0;

    for (i, ch) in s.chars().enumerate() {
        let i = i as i32;
        if ch == '(' {
            stack.push(i);
        } else {
            stack.pop();
            if stack.is_empty() {
                stack.push(i);
            } else {
                max_len = max_len.max(i - stack[stack.len() - 1]);
            }
        }
    }

    max_len
}

fn main() {
    println!("{:?}", longest_valid_parentheses(")()())"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(longest_valid_parentheses("(()"), 2);
        assert_eq!(longest_valid_parentheses(")()())"), 4);
        assert_eq!(longest_valid_parentheses(""), 0);
        assert_eq!(longest_valid_parentheses("()(()"), 2);
        assert_eq!(longest_valid_parentheses("()()"), 4);
    }
}
