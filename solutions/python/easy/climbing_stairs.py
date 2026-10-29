def climb_stairs(n: int) -> int:
    if n <= 2:
        return n
    a, b = 1, 2
    for _ in range(3, n + 1):
        a, b = b, a + b
    return b


print(climb_stairs(2));  # 2
print(climb_stairs(3));  # 3
print(climb_stairs(4));  # 5
print(climb_stairs(5));  # 8
print(climb_stairs(1));  # 1
