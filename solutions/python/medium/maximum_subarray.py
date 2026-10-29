def max_sub_array(nums: list[int]) -> int:
    current_sum = nums[0]
    max_sum = nums[0]
    for num in nums[1:]:
        current_sum = max(num, current_sum + num)
        max_sum = max(max_sum, current_sum)
    return max_sum


print(max_sub_array([-2, 1, -3, 4, -1, 2, 1, -5, 4]));  # 6
print(max_sub_array([1]));  # 1
print(max_sub_array([5, 4, -1, 7, 8]));  # 23
print(max_sub_array([-1, -2, -3]));  # -1
print(max_sub_array([0]));  # 0
