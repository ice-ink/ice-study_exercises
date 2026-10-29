# Container With Most Water — medium
# Given bar heights, return the max water a container between two bars can hold.


def max_area(height: list[int]) -> int:
    left, right = 0, len(height) - 1
    best = 0

    while left < right:
        water = (right - left) * min(height[left], height[right])
        best = max(best, water)

        if height[left] < height[right]:
            left += 1
        else:
            right -= 1

    return best


# Quick checks
print(max_area([1, 8, 6, 2, 5, 4, 8, 3, 7]));  # 49
print(max_area([1, 1]));                       # 1
print(max_area([4, 3, 2, 1, 4]));              # 16
print(max_area([1, 2, 1]));                    # 2
