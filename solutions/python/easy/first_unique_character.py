def first_uniq_char(s: str) -> int:
    counts: dict[str, int] = {}
    for ch in s:
        counts[ch] = counts.get(ch, 0) + 1
    for i, ch in enumerate(s):
        if counts[ch] == 1:
            return i
    return -1


print(first_uniq_char("leetcode"));  # 0
print(first_uniq_char("loveleetcode"));  # 2
print(first_uniq_char("aabb"));  # -1
