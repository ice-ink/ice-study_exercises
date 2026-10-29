from collections import deque


def max_sliding_window(nums: list[int], k: int) -> list[int]:
    n = len(nums)
    result: list[int] = []
    # deque holds indices; the values at those indices are strictly decreasing.
    indices = deque()

    for i in range(n):
        # Drop indices that have slid out of the window's left edge.
        if indices and indices[0] < i - k + 1:
            indices.popleft()

        # Pop from the back while the incoming value is >= the back's value.
        while indices and nums[indices[-1]] <= nums[i]:
            indices.pop()

        indices.append(i)

        # Once the first window is full, the front of the deque is the maximum.
        if i >= k - 1:
            result.append(nums[indices[0]])

    return result


print(max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3));  # [3, 3, 5, 5, 6, 7]
print(max_sliding_window([1], 1));  # [1]
print(max_sliding_window([1, -1], 1));  # [1, -1]
print(max_sliding_window([7, 2, 4], 2));  # [7, 4]
