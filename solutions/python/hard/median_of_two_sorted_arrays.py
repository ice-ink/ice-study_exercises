def find_median_sorted_arrays(nums1: list[int], nums2: list[int]) -> float:
    if len(nums1) > len(nums2):
        return find_median_sorted_arrays(nums2, nums1)

    m = len(nums1)
    n = len(nums2)
    total = m + n
    half = (total + 1) // 2

    low = 0
    high = m

    while low <= high:
        i = (low + high) // 2
        j = half - i

        left1 = -float("inf") if i == 0 else nums1[i - 1]
        right1 = float("inf") if i == m else nums1[i]
        left2 = -float("inf") if j == 0 else nums2[j - 1]
        right2 = float("inf") if j == n else nums2[j]

        if left1 <= right2 and left2 <= right1:
            if total % 2 == 1:
                return float(max(left1, left2))
            return (max(left1, left2) + min(right1, right2)) / 2.0
        elif left1 > right2:
            high = i - 1
        else:
            low = i + 1

    raise ValueError("input arrays are not sorted")


print(find_median_sorted_arrays([1, 3], [2]));  # 2.0
print(find_median_sorted_arrays([1, 2], [3, 4]));  # 2.5
print(find_median_sorted_arrays([0, 0], [0, 0]));  # 0.0
print(find_median_sorted_arrays([], [1]));  # 1.0
print(find_median_sorted_arrays([2], []));  # 2.0
