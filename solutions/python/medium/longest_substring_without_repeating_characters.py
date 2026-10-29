def length_of_longest_substring(s: str) -> int:
    last_seen: dict[str, int] = {}
    longest = 0
    left = 0
    for right, char in enumerate(s):
        if char in last_seen and last_seen[char] >= left:
            left = last_seen[char] + 1
        last_seen[char] = right
        longest = max(longest, right - left + 1)
    return longest


print(length_of_longest_substring("abcabcbb"));  # 3
print(length_of_longest_substring("bbbbb"));  # 1
print(length_of_longest_substring("pwwkew"));  # 3
print(length_of_longest_substring(""));  # 0
print(length_of_longest_substring(" "));  # 1
