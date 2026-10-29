// Longest Substring Without Repeating Characters — medium
// Return the length of the longest substring with no repeating characters.

function lengthOfLongestSubstring(s: string): number {
  const lastSeen = new Map<string, number>();
  let left = 0;
  let maxLength = 0;

  for (let right = 0; right < s.length; right++) {
    const ch = s.charAt(right);
    const seenAt = lastSeen.get(ch);
    if (seenAt !== undefined && seenAt >= left) {
      left = seenAt + 1;
    }
    lastSeen.set(ch, right);
    maxLength = Math.max(maxLength, right - left + 1);
  }

  return maxLength;
}

// Quick checks
console.log(lengthOfLongestSubstring("abcabcbb")); // 3
console.log(lengthOfLongestSubstring("bbbbb"));    // 1
console.log(lengthOfLongestSubstring("pwwkew"));   // 3
console.log(lengthOfLongestSubstring(""));         // 0
console.log(lengthOfLongestSubstring("aab"));      // 2
