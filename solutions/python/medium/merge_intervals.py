# Merge Intervals — medium
# Merge all overlapping intervals [start, end] into disjoint intervals.


def merge(intervals: list[list[int]]) -> list[list[int]]:
    ordered = sorted(intervals, key=lambda x: x[0])
    merged: list[list[int]] = []

    for start, end in ordered:
        if not merged or start > merged[-1][1]:
            merged.append([start, end])
        else:
            merged[-1][1] = max(merged[-1][1], end)

    return merged


# Quick checks
print(merge([[1, 3], [2, 6], [8, 10], [15, 18]]));  # [[1, 6], [8, 10], [15, 18]]
print(merge([[1, 4], [4, 5]]));                      # [[1, 5]]
print(merge([[1, 4], [0, 4]]));                      # [[0, 4]]
print(merge([[1, 4], [5, 6]]));                      # [[1, 4], [5, 6]]
print(merge([]));                                    # []
