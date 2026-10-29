# Three Sum — medium
# Return all unique triplets whose sum is 0.


def three_sum(nums: list[int]) -> list[list[int]]:
    nums.sort()
    result: list[list[int]] = []
    n = len(nums)

    for i in range(n - 2):
        if i > 0 and nums[i] == nums[i - 1]:
            continue

        left, right = i + 1, n - 1
        while left < right:
            total = nums[i] + nums[left] + nums[right]
            if total < 0:
                left += 1
            elif total > 0:
                right -= 1
            else:
                result.append([nums[i], nums[left], nums[right]])
                left += 1
                right -= 1
                while left < right and nums[left] == nums[left - 1]:
                    left += 1
                while left < right and nums[right] == nums[right + 1]:
                    right -= 1

    return result


# Quick checks
print(three_sum([-1, 0, 1, 2, -1, -4]));  # [[-1, -1, 2], [-1, 0, 1]]
print(three_sum([0, 1, 1]));              # []
print(three_sum([0, 0, 0]));              # [[0, 0, 0]]
print(three_sum([-2, 0, 0, 2, 2]));       # [[-2, 0, 2]]
