def max_profit(prices: list[int]) -> int:
    min_price = float("inf")
    profit = 0
    for price in prices:
        if price < min_price:
            min_price = price
        elif price - min_price > profit:
            profit = price - min_price
    return profit


print(max_profit([7, 1, 5, 3, 6, 4]));  # 5
print(max_profit([7, 6, 4, 3, 1]));  # 0
print(max_profit([1, 2]));  # 1
print(max_profit([2, 4, 1]));  # 2
