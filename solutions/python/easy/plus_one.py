def plus_one(digits: list[int]) -> list[int]:
    for i in range(len(digits) - 1, -1, -1):
        if digits[i] < 9:
            digits[i] += 1
            return digits
        digits[i] = 0
    return [1] + digits


print(plus_one([1, 2, 3]));  # [1, 2, 4]
print(plus_one([4, 3, 2, 1]));  # [4, 3, 2, 2]
print(plus_one([9]));  # [1, 0]
print(plus_one([9, 9, 9]));  # [1, 0, 0, 0]
print(plus_one([0]));  # [1]
