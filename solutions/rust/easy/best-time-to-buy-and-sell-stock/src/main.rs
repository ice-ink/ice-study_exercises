pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut best = 0;

    for price in prices {
        min_price = min_price.min(price);
        best = best.max(price - min_price);
    }

    best
}

fn main() {
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4])); // 5
    println!("{}", max_profit(vec![7, 6, 4, 3, 1]));    // 0
    println!("{}", max_profit(vec![1, 2]));             // 1
    println!("{}", max_profit(vec![1]));                // 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(vec![1, 2]), 1);
        assert_eq!(max_profit(vec![2, 4, 1]), 2);
        assert_eq!(max_profit(vec![1]), 0);
    }
}
