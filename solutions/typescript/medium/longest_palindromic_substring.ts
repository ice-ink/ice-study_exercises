// Longest Palindromic Substring — medium
// Return the longest palindromic substring in s.

function longestPalindrome(s: string): string {
  let start = 0;
  let maxLen = 0;

  const expandAroundCenter = (left: number, right: number): void => {
    while (left >= 0 && right < s.length && s.charAt(left) === s.charAt(right)) {
      const len = right - left + 1;
      if (len > maxLen) {
        maxLen = len;
        start = left;
      }
      left--;
      right++;
    }
  };

  for (let i = 0; i < s.length; i++) {
    expandAroundCenter(i, i); // odd-length palindromes
    expandAroundCenter(i, i + 1); // even-length palindromes
  }

  return s.slice(start, start + maxLen);
}

// Quick checks
console.log(longestPalindrome("babad"));   // "bab"
console.log(longestPalindrome("cbbd"));    // "bb"
console.log(longestPalindrome("racecar")); // "racecar"
console.log(longestPalindrome("a"));       // "a"
console.log(longestPalindrome(""));        // ""
