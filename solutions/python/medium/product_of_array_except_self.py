def product_except_self(nums: list[int]) -> list[int]:
    result = [1] * len(nums)

    prefix = 1
    for i in range(len(nums)):
        result[i] = prefix
        prefix *= nums[i]

    suffix = 1
    for i in range(len(nums) - 1, -1, -1):
        result[i] *= suffix
        suffix *= nums[i]

    return result


print(product_except_self([1, 2, 3, 4]));  # [24, 12, 8, 6]
print(product_except_self([-1, 1, 0, -3, 3]));  # [0, 0, 9, 0, 0]
print(product_except_self([2, 3]));  # [3, 2]
print(product_except_self([0, 0]));  # [0, 0]
