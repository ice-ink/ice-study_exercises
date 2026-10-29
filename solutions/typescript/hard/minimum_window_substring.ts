// Minimum Window Substring — hard
// Return the minimum-length substring of s that contains all characters of t (with counts).

function minWindow(s: string, t: string): string {
  if (s.length < t.length) return "";

  const need = new Map<string, number>();
  for (const ch of t) {
    need.set(ch, (need.get(ch) ?? 0) + 1);
  }

  const required = need.size; // distinct characters we must satisfy
  const window = new Map<string, number>();
  let have = 0; // distinct characters currently satisfied
  let left = 0;
  let bestLeft = 0;
  let bestLen = Infinity;

  for (let right = 0; right < s.length; right++) {
    const ch = s[right]!;
    window.set(ch, (window.get(ch) ?? 0) + 1);

    if (need.has(ch) && (window.get(ch) ?? 0) === (need.get(ch) ?? 0)) {
      have++;
    }

    // Shrink from the left while the window still satisfies all needs.
    while (have === required) {
      const len = right - left + 1;
      if (len < bestLen) {
        bestLen = len;
        bestLeft = left;
      }

      const leftCh = s[left]!;
      window.set(leftCh, (window.get(leftCh) ?? 0) - 1);
      if (need.has(leftCh) && (window.get(leftCh) ?? 0) < (need.get(leftCh) ?? 0)) {
        have--;
      }
      left++;
    }
  }

  return bestLen === Infinity ? "" : s.slice(bestLeft, bestLeft + bestLen);
}

// Quick checks
console.log(minWindow("ADOBECODEBANC", "ABC")); // "BANC"
console.log(minWindow("a", "a")); // "a"
console.log(minWindow("a", "aa")); // ""
console.log(minWindow("ab", "b")); // "b"
