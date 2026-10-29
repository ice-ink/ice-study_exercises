def remove_duplicates(nums: list[int]) -> int:
    if not nums:
        return 0
    write = 1
    for read in range(1, len(nums)):
        if nums[read] != nums[read - 1]:
            nums[write] = nums[read]
            write += 1
    return write


print(remove_duplicates([1, 1, 2]));  # 2
print(remove_duplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));  # 5
print(remove_duplicates([1, 1, 1]));  # 1
print(remove_duplicates([]));  # 0
