def group_anagrams(strs: list[str]) -> list[list[str]]:
    groups: dict[str, list[str]] = {}
    for word in strs:
        key = "".join(sorted(word))
        groups.setdefault(key, []).append(word)
    return list(groups.values())


print(group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"]));  # [["eat", "tea", "ate"], ["tan", "nat"], ["bat"]]
print(group_anagrams([""]));  # [[""]]
print(group_anagrams(["a"]));  # [["a"]]
print(group_anagrams(["cab", "tin", "pew", "duh", "may", "ill", "buy", "bar", "max", "doc"]));  # [["cab"], ["tin"], ["pew"], ["duh"], ["may"], ["ill"], ["buy"], ["bar"], ["max"], ["doc"]]
