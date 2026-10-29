from collections import Counter


def is_anagram(s: str, t: str) -> bool:
    return Counter(s) == Counter(t)


print(is_anagram("anagram", "nagaram"));  # True
print(is_anagram("rat", "car"));  # False
print(is_anagram("a", "ab"));  # False
print(is_anagram("listen", "silent"));  # True
