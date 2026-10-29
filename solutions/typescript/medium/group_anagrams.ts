// Group Anagrams — medium
// Group strings that are anagrams of each other.

function groupAnagrams(strs: string[]): string[][] {
  const groups = new Map<string, string[]>();

  for (const str of strs) {
    const key = str.split("").sort().join("");
    const existing = groups.get(key);
    if (existing !== undefined) {
      existing.push(str);
    } else {
      groups.set(key, [str]);
    }
  }

  return Array.from(groups.values());
}

// Quick checks
console.log(groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]));
// [["eat","tea","ate"],["tan","nat"],["bat"]]
console.log(groupAnagrams([""]));  // [[""]]
console.log(groupAnagrams(["a"])); // [["a"]]
console.log(groupAnagrams(["ab", "ba", "abc"])); // [["ab","ba"],["abc"]]
