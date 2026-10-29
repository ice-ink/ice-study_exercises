def longest_palindrome(s: str) -> str:
    start = 0
    max_len = 0

    def expand_around_center(left: int, right: int) -> None:
        nonlocal start, max_len
        while left >= 0 and right < len(s) and s[left] == s[right]:
            length = right - left + 1
            if length > max_len:
                max_len = length
                start = left
            left -= 1
            right += 1

    for i in range(len(s)):
        expand_around_center(i, i)  # odd-length palindromes
        expand_around_center(i, i + 1)  # even-length palindromes

    return s[start:start + max_len]


print(longest_palindrome("babad"));  # "bab"
print(longest_palindrome("cbbd"));  # "bb"
print(longest_palindrome("racecar"));  # "racecar"
print(longest_palindrome("a"));  # "a"
print(longest_palindrome(""));  # ""
