# Rotate Array — medium
# Rotate the list to the right by k steps, in place.


def rotate(nums: list[int], k: int) -> None:
    n = len(nums)
    if n == 0:
        return
    k %= n

    def reverse(start: int, end: int) -> None:
        while start < end:
            nums[start], nums[end] = nums[end], nums[start]
            start += 1
            end -= 1

    reverse(0, n - 1)
    reverse(0, k - 1)
    reverse(k, n - 1)


# Quick checks
a1 = [1, 2, 3, 4, 5, 6, 7]
rotate(a1, 3)
print(a1);  # [5, 6, 7, 1, 2, 3, 4]

a2 = [-1, -100, 3, 99]
rotate(a2, 2)
print(a2);  # [3, 99, -1, -100]

a3 = [1, 2, 3]
rotate(a3, 4)
print(a3);  # [3, 1, 2]

a4 = [1, 2, 3, 4]
rotate(a4, 0)
print(a4);  # [1, 2, 3, 4]
