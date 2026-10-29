def contains_duplicate(nums: list[int]) -> bool:
    seen = set()
    for num in nums:
        if num in seen:
            return True
        seen.add(num)
    return False


print(contains_duplicate([1, 2, 3, 1]));  # True
print(contains_duplicate([1, 2, 3, 4]));  # False
print(contains_duplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));  # True
