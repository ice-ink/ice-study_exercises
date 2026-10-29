from collections import Counter


def top_k_frequent(nums: list[int], k: int) -> list[int]:
    counts = Counter(nums)
    return [num for num, _ in counts.most_common(k)]


print(top_k_frequent([1, 1, 1, 2, 2, 3], 2));  # [1, 2]
print(top_k_frequent([1], 1));  # [1]
print(top_k_frequent([4, 1, -1, 2, -1, 2, 3], 2));  # [-1, 2]
print(top_k_frequent([1, 2, 2, 3, 3, 3], 3));  # [3, 2, 1]
