// First Unique Character in a String — easy
// Return the index of the first non-repeating character, or -1 if none.

function firstUniqChar(s: string): number {
  const counts = new Array<number>(26).fill(0);

  for (const ch of s) {
    counts[ch.charCodeAt(0) - 97]! += 1;
  }

  for (let i = 0; i < s.length; i++) {
    if (counts[s.charCodeAt(i) - 97]! === 1) return i;
  }

  return -1;
}

// Quick checks
console.log(firstUniqChar("leetcode")); // 0
console.log(firstUniqChar("loveleetcode")); // 2
console.log(firstUniqChar("aabb")); // -1
console.log(firstUniqChar("")); // -1
