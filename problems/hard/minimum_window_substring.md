# Minimum Window Substring (deepseek generated)

## Problem

Given two strings `s` and `t`, return the minimum-length contiguous substring of `s` that contains every character of `t`, including duplicates (i.e. the substring's counts must be at least the counts in `t`). If no such substring exists, return the empty string `""`. If there are several, return any one of them.

## Examples

| Input | Output | Why |
|---|---|---|
| `s = "ADOBECODEBANC"`, `t = "ABC"` | `"BANC"` | Shortest substring containing A, B, and C |
| `s = "a"`, `t = "a"` | `"a"` | The whole string is the answer |
| `s = "a"`, `t = "aa"` | `""` | `s` cannot supply two `a`s |
| `s = "ab"`, `t = "b"` | `"b"` | Single required character |
| `s = "ABBAC"`, `t = "ABC"` | `"BAC"` | Must shrink past extra A and B |

## Constraints

- `1 <= s.length, t.length <= 10^5`
- `s` and `t` consist of uppercase and lowercase English letters.

## Things to think about before coding

- What two conditions make a window "valid"? How do you track them without rescanning the window on every move?
- A valid window might still be shrinkable from the left. When do you stop shrinking?
- How do you handle duplicate characters in `t` (e.g. `t = "aa"`)?
- Target O(m + n) time and O(alphabet) space — a fixed-size alphabet keeps the maps tiny.

Give it a try before scrolling further.

---

## Solution

### Approach

Use a **sliding window** with two frequency maps: one for what `t` needs and one for what the current window has, plus a `have` counter of how many *distinct* required characters are satisfied. Expand the right edge to add characters; whenever the window is valid, shrink the left edge while it stays valid, recording the best window seen.

1. Build `need` from `t` and set `required = need.size`.
2. Expand `right`, adding `s[right]` to `window` and bumping `have` when a required character's count exactly reaches its need.
3. While `have === required`, the window is valid: record it if it is the shortest, then remove `s[left]` and advance `left`, decrementing `have` when a required count drops below its need.

**Time:** O(m + n), each character enters and leaves the window at most once. **Space:** O(alphabet), bounded by the number of distinct characters.

### TypeScript

```typescript
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
```

### Rust

```rust
use std::collections::HashMap;

pub fn min_window(s: &str, t: &str) -> String {
    if s.len() < t.len() {
        return String::new();
    }

    // Constraints guarantee ASCII (English letters), so byte indexing is safe.
    let s_bytes = s.as_bytes();
    let mut need: HashMap<u8, usize> = HashMap::new();
    for &b in t.as_bytes() {
        *need.entry(b).or_insert(0) += 1;
    }

    let required = need.len(); // distinct characters we must satisfy
    let mut window: HashMap<u8, usize> = HashMap::new();
    let mut have = 0usize; // distinct characters currently satisfied
    let mut left = 0usize;
    let mut best_left = 0usize;
    let mut best_len = usize::MAX;

    for right in 0..s_bytes.len() {
        let b = s_bytes[right];
        *window.entry(b).or_insert(0) += 1;

        if window.get(&b) == need.get(&b) {
            have += 1;
        }

        // Shrink from the left while the window still satisfies all needs.
        while have == required {
            let len = right - left + 1;
            if len < best_len {
                best_len = len;
                best_left = left;
            }

            let lb = s_bytes[left];
            if let Some(cnt) = window.get_mut(&lb) {
                *cnt -= 1;
            }
            if window.get(&lb) < need.get(&lb) {
                have -= 1;
            }
            left += 1;
        }
    }

    if best_len == usize::MAX {
        String::new()
    } else {
        s[best_left..best_left + best_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(min_window("a", "a"), "a");
        assert_eq!(min_window("a", "aa"), "");
        assert_eq!(min_window("ab", "b"), "b");
    }
}
```

### Notes on the two implementations

- The `have`/`required` trick counts *distinct satisfied characters*, not total characters, so the maps stay proportional to the alphabet instead of the window length.
- TypeScript reads `Map.get()` which returns `number | undefined`; the `?? 0` default turns a missing key into `0`. Rust uses `Option<&usize>` and compares two `Option`s directly.
- Rust indexes the string as **bytes** (`u8`), which is safe only because the constraint limits input to ASCII English letters — that is why byte slicing `s[a..b]` cannot panic.

## Likely interviewer follow-ups

1. **Return the indices:** how would you return `[start, end)` instead of the substring itself?
2. **Multiple answers:** does your code return a deterministic result when two shortest windows tie, and would you change that?
3. **Non-ASCII input:** what breaks if `s` contains multi-byte UTF-8 characters, and how would you fix it (e.g. `Vec<char>`)?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Count the characters `t` needs (`need`) and how many distinct characters that is (`required`).
2. Slide a right pointer across `s`, adding each character to a `window` count.
3. Track `have`, the number of distinct required characters whose window count is at least the need.
4. When `have === required`, the window is valid: shrink the left pointer while it stays valid, remembering the shortest window.

## TypeScript, piece by piece

```typescript
const need = new Map<string, number>();
for (const ch of t) {
  need.set(ch, (need.get(ch) ?? 0) + 1);
}
```
`need` maps each character of `t` to how many times it appears. `need.get(ch)` returns `number | undefined`; `?? 0` gives a safe `0` the first time we see a character, then we add 1 and store it back.

```typescript
if (need.has(ch) && (window.get(ch) ?? 0) === (need.get(ch) ?? 0)) {
  have++;
}
```
We bump `have` only when a *required* character's window count **exactly reaches** its need. The `===` means an overshoot (count goes above need) does not re-increment, which is essential for the counter to be accurate.

```typescript
while (have === required) {
  const len = right - left + 1;
  if (len < bestLen) { bestLen = len; bestLeft = left; }
  // ... remove s[left], advance left
}
```
Once the window is valid we try to make it smaller. `len < bestLen` (strict `<`) keeps the *first* shortest window. Each shrink removes the left character and re-checks validity on the next loop iteration.

```typescript
return bestLen === Infinity ? "" : s.slice(bestLeft, bestLeft + bestLen);
```
`Infinity` is the "no valid window found yet" sentinel; if it survives the whole scan we return `""`. Otherwise `slice(start, end)` extracts the best window, where `end` is exclusive.

## Rust, piece by piece

```rust
let s_bytes = s.as_bytes();
let mut need: HashMap<u8, usize> = HashMap::new();
for &b in t.as_bytes() {
    *need.entry(b).or_insert(0) += 1;
}
```
We treat both strings as byte slices. `entry(b).or_insert(0)` returns a `&mut usize` pointing at the key's value (inserting `0` if absent), and `*... += 1` mutates it through the reference.

```rust
if window.get(&b) == need.get(&b) {
    have += 1;
}
```
Both `.get(...)` calls return `Option<&usize>`. Comparing them does the right thing: a character absent from `need` gives `Some(count) == None`, which is `false`, so we never count characters `t` does not care about.

```rust
if let Some(cnt) = window.get_mut(&lb) {
    *cnt -= 1;
}
if window.get(&lb) < need.get(&lb) {
    have -= 1;
}
```
`get_mut` returns a mutable reference to decrement the window count. The `<` between two `Option<&usize>` values is `true` when the window count drops below the need (`None` sorts below `Some`, but here the guard only matters for required characters).

```rust
if best_len == usize::MAX {
    String::new()
} else {
    s[best_left..best_left + best_len].to_string()
}
```
`usize::MAX` is Rust's "infinity" sentinel. `&str[..]` slices by byte offsets and returns a `&str`; `.to_string()` turns it into an owned `String`, which is what the function must return.

## Trace: `s = "ABBAC"`, `t = "ABC"` (should be `"BAC"`)

`need = {A:1, B:1, C:1}`, so `required = 3`.

| Step | right | ch | Action | have/required | best window |
|---|---|---|---|---|---|
| 1 | 0 | A | add A | 1/3 | — |
| 2 | 1 | B | add B | 2/3 | — |
| 3 | 2 | B | add B (overshoot) | 2/3 | — |
| 4 | 3 | A | add A (overshoot) | 2/3 | — |
| 5 | 4 | C | add C → window valid | 3/3 | `(0,5)` "ABBAC" |
| 6 | — | — | shrink: drop A, still valid | 3/3 | `(1,4)` "BBAC" |
| 7 | — | — | shrink: drop B, still valid | 3/3 | `(2,3)` "BAC" |
| 8 | — | — | shrink: drop B → not valid, stop | 2/3 | `(2,3)` "BAC" |

The final best window is `(2,3)`, which is `s[2..5] = "BAC"`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Frequency map | `Map<string, number>` | `HashMap<u8, usize>` |
| Missing key | `map.get(k) ?? 0` | `entry(k).or_insert(0)` |
| Compare counts | `a === b` on `number` | `a == b` on `Option<&usize>` |
| "No answer yet" | `Infinity` | `usize::MAX` |
| Extract substring | `s.slice(start, end)` | `&s[start..end]` |
| Empty string | `""` | `String::new()` |
