// Valid Anagram — easy
// Return true if t is an anagram of s (same letters, same counts).

function isAnagram(s: string, t: string): boolean {
  if (s.length !== t.length) return false;

  // 26 slots for 'a'..'z'; index = charCode - 97.
  const counts = new Array<number>(26).fill(0);

  for (let i = 0; i < s.length; i++) {
    counts[s.charCodeAt(i) - 97]! += 1;
    counts[t.charCodeAt(i) - 97]! -= 1;
  }

  return counts.every((c) => c === 0);
}

// Quick checks
console.log(isAnagram("anagram", "nagaram")); // true
console.log(isAnagram("rat", "car")); // false
console.log(isAnagram("a", "ab")); // false
console.log(isAnagram("", "")); // true
