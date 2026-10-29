def trap(height: list[int]) -> int:
    left = 0
    right = len(height) - 1
    left_max = 0
    right_max = 0
    water = 0

    while left < right:
        left_height = height[left]
        right_height = height[right]

        if left_height < right_height:
            if left_height >= left_max:
                left_max = left_height
            else:
                water += left_max - left_height
            left += 1
        else:
            if right_height >= right_max:
                right_max = right_height
            else:
                water += right_max - right_height
            right -= 1

    return water


print(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));  # 6
print(trap([4, 2, 0, 3, 2, 5]));  # 9
print(trap([1, 2, 3]));  # 0
print(trap([3, 2, 1]));  # 0
print(trap([]));  # 0
