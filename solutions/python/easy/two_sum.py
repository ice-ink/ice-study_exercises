def two_sum(nums: list[int], target: int) -> list[int]:
    seen: dict[int, int] = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in seen:
            return [seen[complement], i]
        seen[num] = i
    return []


print(two_sum([2, 7, 11, 15], 9));  # [0, 1]
print(two_sum([3, 2, 4], 6));  # [1, 2]
print(two_sum([3, 3], 6));  # [0, 1]
print(two_sum([1, 2, 3], 10));  # []
