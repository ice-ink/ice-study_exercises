pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;

    for a in 1..=amount {
        for &coin in &coins {
            let coin = coin as usize;
            if coin <= a && dp[a - coin] != i32::MAX {
                dp[a] = dp[a].min(dp[a - coin] + 1);
            }
        }
    }

    if dp[amount] == i32::MAX {
        -1
    } else {
        dp[amount]
    }
}

fn main() {
    println!("{:?}", coin_change(vec![1, 2, 5], 11));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
        assert_eq!(coin_change(vec![2, 5, 10, 1], 27), 4);
        assert_eq!(coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
