# Longest Palindromic Substring (deepseek generated)

## Problem

Given a string `s`, return the **longest palindromic substring** in `s`. A palindrome reads the same forwards and backwards. If there is more than one answer, returning any of the longest palindromes is fine.

## Examples

| Input | Output | Why |
|---|---|---|
| `"babad"` | `"bab"` | Both `"bab"` and `"aba"` are length 3; either is acceptable |
| `"cbbd"` | `"bb"` | The longest palindrome has even length |
| `"racecar"` | `"racecar"` | The whole string is a palindrome |
| `"a"` | `"a"` | A single character is trivially a palindrome |
| `"ac"` | `"a"` | No two-character match, so any length-1 substring works |
| `""` | `""` | Empty string is a palindrome |

## Constraints

- `1 <= s.length <= 1000`
- `s` consists of only digits and English letters (ASCII), so one character is one byte.

## Things to think about before coding

- A palindrome grows outward from its center. How many *kinds* of centers are there? (Hint: odd-length vs even-length.)
- For each center, how far can you expand while the two characters still match?
- How do you track the best (longest) palindrome found so far without storing every candidate?
- Aim for O(n²) time and O(1) extra space.

Give it a try before scrolling further.

---

## Solution

### Approach

Treat every position (and every gap between positions) as a potential center and **expand outward** while the two characters match. Odd-length palindromes expand from a single character; even-length ones expand from a pair. Keep the start index and length of the longest palindrome found.

1. Initialize `start = 0` and `max_len = 0`.
2. For every index `i`, expand from center `(i, i)` (odd) and from `(i, i + 1)` (even).
3. Expanding compares the two outer characters; while they match, widen `left`/`right` by one.
4. When an expansion finds a longer palindrome, record its start and length.
5. Return the slice `s[start .. start + max_len]`.

**Time:** O(n²), since there are ~2n centers and each expansion is O(n). **Space:** O(1) beyond the returned string.

### TypeScript

```typescript
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
```

### Rust

```rust
pub fn longest_palindrome(s: &str) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len() as i32;

    let mut start: i32 = 0;
    let mut max_len: i32 = 0;

    for i in 0..n {
        let (s1, l1) = expand_around_center(bytes, i, i);
        let (s2, l2) = expand_around_center(bytes, i, i + 1);

        if l1 > max_len {
            max_len = l1;
            start = s1;
        }
        if l2 > max_len {
            max_len = l2;
            start = s2;
        }
    }

    s[start as usize..(start + max_len) as usize].to_string()
}

fn expand_around_center(bytes: &[u8], mut left: i32, mut right: i32) -> (i32, i32) {
    let n = bytes.len() as i32;
    while left >= 0 && right < n && bytes[left as usize] == bytes[right as usize] {
        left -= 1;
        right += 1;
    }
    (left + 1, right - left - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(longest_palindrome("babad"), "bab");
        assert_eq!(longest_palindrome("cbbd"), "bb");
        assert_eq!(longest_palindrome("racecar"), "racecar");
        assert_eq!(longest_palindrome("a"), "a");
        assert_eq!(longest_palindrome(""), "");
    }
}
```

### Notes on the two implementations

- Both track `start` and `max_len` (a length) and reconstruct the answer with one final slice, so no O(n²) list of candidate strings is ever stored.
- The Rust version works on bytes and stores indices as `i32` so `left` can go below 0 during expansion without underflowing `usize`; it is safe because the input is ASCII.
- `charAt` is used in TypeScript so `s.charAt(left) === s.charAt(right)` compares plain strings even with `noUncheckedIndexedAccess` enabled.

## Likely interviewer follow-ups

1. **Return all:** how would you return *all* longest palindromic substrings if there are several?
2. **Linear time:** what is Manacher's algorithm, and how does it get O(n) time?
3. **Count, not find:** how would you count the total number of palindromic substrings (not just the longest)?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. A palindrome is determined by its center: either one character (odd length) or a gap between two characters (even length).
2. For each possible center, expand outward one character at a time while the two ends match.
3. Track the longest match's start position and length.
4. At the end, cut that slice out of the original string and return it.

## TypeScript, piece by piece

```typescript
const expandAroundCenter = (left: number, right: number): void => {
```
This is an **arrow function** assigned to a `const`. It takes two numbers (the center boundaries) and returns `void` because it doesn't produce a value — it only updates `start` and `maxLen` through the closure.

```typescript
while (left >= 0 && right < s.length && s.charAt(left) === s.charAt(right)) {
```
Three conditions must all hold: `left` hasn't run off the left edge, `right` hasn't run off the right edge, and the two characters match. As soon as any fails, we stop expanding.

```typescript
const len = right - left + 1;
if (len > maxLen) {
  maxLen = len;
  start = left;
}
```
`len` is the length of the palindrome spanning `left..right` (inclusive). The `>` (not `>=`) keeps the *first* longest palindrome found, which makes the output deterministic.

```typescript
left--;
right++;
```
Move both pointers outward to test the next, wider pair. This mutates the parameters, which is fine because they are local copies passed by value.

```typescript
expandAroundCenter(i, i);     // odd
expandAroundCenter(i, i + 1); // even
return s.slice(start, start + maxLen);
```
Odd palindromes start with both pointers on the same character; even palindromes start on two adjacent characters. After trying every center, `slice` extracts the winning substring.

## Rust, piece by piece

```rust
let bytes = s.as_bytes();
```
Rust won't let you write `s[i]` on a `&str` (UTF-8 characters vary in width). `.as_bytes()` gives a `&[u8]` view so we can index directly — safe here because the problem guarantees ASCII (one byte per character).

```rust
fn expand_around_center(bytes: &[u8], mut left: i32, mut right: i32) -> (i32, i32) {
```
This helper returns a **tuple** `(start, length)`. `left`/`right` are `i32` (signed) on purpose: during expansion `left` can become `-1`, and `usize` cannot represent that without panicking.

```rust
while left >= 0 && right < n && bytes[left as usize] == bytes[right as usize] {
    left -= 1;
    right += 1;
}
(left + 1, right - left - 1)
```
Inside the loop the indices are valid, so `left as usize` (and `right as usize`) are safe casts from `i32`. When the loop stops, the actual palindrome sits just inside the final bounds: it starts at `left + 1` and has length `right - left - 1`.

```rust
let (s1, l1) = expand_around_center(bytes, i, i);
let (s2, l2) = expand_around_center(bytes, i, i + 1);
```
**Tuple destructuring**: the two returned values are unpacked into named variables in one statement. `i, i` is the odd center; `i, i + 1` is the even center.

```rust
s[start as usize..(start + max_len) as usize].to_string()
```
`start` and `max_len` are `i32`, but slicing needs `usize`, so both are cast back. Slicing a `&str` yields a `&str`, and `.to_string()` copies it into an owned `String` — the function's return type.

## Trace: `"cbbd"` (should be `"bb"`)

| Step | i | center(s) tried | start, max_len after |
|---|---|---|---|
| 1 | 0 | odd `(0,0)` → `"c"` (len 1) | 0, 1 |
| 2 | 0 | even `(0,1)` → `c` vs `b`, stop (len 0) | 0, 1 |
| 3 | 1 | odd `(1,1)` → `"b"` (len 1) | 0, 1 |
| 4 | 1 | even `(1,2)` → `b`=`b`, then `c`≠`d` → `"bb"` (len 2) | 1, 2 |
| 5 | 2 | odd `(2,2)` → `"b"`; even `(2,3)` → `b` vs `d`, stop | 1, 2 |
| 6 | 3 | odd `(3,3)` → `"d"`; even `(3,4)` → out of bounds | 1, 2 |

The winning slice is `s[1..3] = "bb"`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Character at index | `s.charAt(i)` | `bytes[i]` (after `as_bytes()`) |
| Signed loop index that can go negative | `number` (fine) | `i32` (must avoid `usize` underflow) |
| Helper return of two values | closure updates outer `let`s | tuple `(i32, i32)` |
| Slice a substring | `s.slice(a, b)` | `s[a..b].to_string()` |
| Decrement | `left--` | `left -= 1` |
| Final return | `return ...;` | last expression, no `return`/`;` |
| Non-empty guarantee | `s.length === 0` handled by loop | `0..n` range naturally empty |
