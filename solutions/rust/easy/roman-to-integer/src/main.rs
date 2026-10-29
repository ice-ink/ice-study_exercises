pub fn roman_to_int(s: &str) -> i32 {
    let mut total = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        let curr = value(c);
        let next = match chars.peek() {
            Some(&n) => value(n),
            None => 0,
        };

        if curr < next {
            total -= curr;
        } else {
            total += curr;
        }
    }

    total
}

fn value(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

fn main() {
    println!("{}", roman_to_int("III"));     // 3
    println!("{}", roman_to_int("LVIII"));   // 58
    println!("{}", roman_to_int("MCMXCIV")); // 1994
    println!("{}", roman_to_int("IV"));      // 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(roman_to_int("III"), 3);
        assert_eq!(roman_to_int("LVIII"), 58);
        assert_eq!(roman_to_int("MCMXCIV"), 1994);
        assert_eq!(roman_to_int("IV"), 4);
        assert_eq!(roman_to_int("IX"), 9);
    }
}
