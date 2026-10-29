from collections import Counter


def min_window(s: str, t: str) -> str:
    if len(s) < len(t):
        return ""

    need = Counter(t)
    window = Counter()

    required = len(need)  # distinct characters we must satisfy
    have = 0  # distinct characters currently satisfied
    left = 0
    best_left = 0
    best_len = float("inf")

    for right, ch in enumerate(s):
        window[ch] += 1

        if ch in need and window[ch] == need[ch]:
            have += 1

        # Shrink from the left while the window still satisfies all needs.
        while have == required:
            length = right - left + 1
            if length < best_len:
                best_len = length
                best_left = left

            left_ch = s[left]
            window[left_ch] -= 1
            if left_ch in need and window[left_ch] < need[left_ch]:
                have -= 1
            left += 1

    return "" if best_len == float("inf") else s[best_left:best_left + best_len]


print(min_window("ADOBECODEBANC", "ABC"));  # "BANC"
print(min_window("a", "a"));  # "a"
print(min_window("a", "aa"));  # ""
print(min_window("ab", "b"));  # "b"
