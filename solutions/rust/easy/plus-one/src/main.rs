pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }

    let mut result = vec![1];
    result.extend_from_slice(&digits);
    result
}

fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3])); // [1, 2, 4]
    println!("{:?}", plus_one(vec![4, 3, 2, 1])); // [4, 3, 2, 2]
    println!("{:?}", plus_one(vec![9])); // [1, 0]
    println!("{:?}", plus_one(vec![9, 9, 9])); // [1, 0, 0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(plus_one(vec![0]), vec![1]);
    }
}
