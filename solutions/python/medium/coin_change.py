# Coin Change — medium
# Return the fewest number of coins that make up the amount, or -1 if it can't be made.


def coin_change(coins: list[int], amount: int) -> int:
    dp = [amount + 1] * (amount + 1)
    dp[0] = 0

    for a in range(1, amount + 1):
        for coin in coins:
            if coin <= a:
                dp[a] = min(dp[a], dp[a - coin] + 1)

    return dp[amount] if dp[amount] != amount + 1 else -1


# Quick checks
print(coin_change([1, 2, 5], 11));            # 3  (5+5+1)
print(coin_change([2], 3));                   # -1
print(coin_change([1], 0));                   # 0
print(coin_change([2, 5, 10, 1], 27));        # 4  (10+10+5+2)
print(coin_change([186, 419, 83, 408], 6249));  # 20
