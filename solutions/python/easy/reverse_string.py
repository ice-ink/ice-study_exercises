def reverse_string(s: list[str]) -> None:
    left, right = 0, len(s) - 1
    while left < right:
        s[left], s[right] = s[right], s[left]
        left += 1
        right -= 1


s1 = ["h", "e", "l", "l", "o"]
reverse_string(s1)
print(s1);  # ['o', 'l', 'l', 'e', 'h']

s2 = ["H", "a", "n", "n", "a", "h"]
reverse_string(s2)
print(s2);  # ['h', 'a', 'n', 'n', 'a', 'H']

s3 = ["a"]
reverse_string(s3)
print(s3);  # ['a']
