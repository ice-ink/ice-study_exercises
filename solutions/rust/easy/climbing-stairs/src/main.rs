pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut first = 1;  // ways to climb 1 step
    let mut second = 2; // ways to climb 2 steps

    for _ in 3..=n {
        let third = first + second;
        first = second;
        second = third;
    }

    second
}

fn main() {
    println!("{}", climb_stairs(2)); // 2
    println!("{}", climb_stairs(3)); // 3
    println!("{}", climb_stairs(4)); // 5
    println!("{}", climb_stairs(5)); // 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}
