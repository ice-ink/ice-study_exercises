# Longest Substring Without Repeating Characters (deepseek generated)

## Problem

Given a string `s`, return the length of the **longest substring** (a contiguous block of characters) that contains no repeating characters. A character may appear at most once inside the substring. For example, in `"abcabcbb"` the answer is `3`, from the substring `"abc"`.

## Examples

| Input | Output | Why |
|---|---|---|
| `"abcabcbb"` | `3` | Longest run without repeats is `"abc"` |
| `"bbbbb"` | `1` | The only duplicate-free windows are single `"b"`s |
| `"pwwkew"` | `3` | `"wke"` is the longest; note `"pwke"` is a *subsequence*, not a substring |
| `"aab"` | `2` | `"ab"` |
| `"dvdf"` | `3` | `"vdf"` — the window must jump past the earlier `d`, not just restart at the duplicate |
| `""` | `0` | Empty string has no characters |

## Constraints

- `0 <= s.length <= 5 * 10^4`
- `s` consists of English letters, digits, symbols and spaces.

## Things to think about before coding

- Keep a window `[left, right]`. What makes the window invalid? The new character at `right` already appearing *inside* the window.
- What data structure can answer "where did I last see this character?" in O(1)?
- When you hit a repeat, move `left` to *just after* the previous occurrence — not just one step forward. Test `"dvdf"`.
- Aim for a single pass: O(n) time.

Give it a try before scrolling further.

---

## Solution

### Approach

Slide a window across the string with two pointers, `left` and `right`. As `right` advances, record each character's most recent index in a hash map. If the current character was last seen at an index that is still inside the window (`>= left`), jump `left` to just past that previous occurrence. The window is then always duplicate-free, so its size `right - left + 1` is a candidate answer.

1. Create an empty map from character → last-seen index, and set `left = 0`, `maxLength = 0`.
2. For each `right` from `0` to `s.length - 1`, read the character `ch`.
3. If `ch` is in the map and its stored index is `>= left`, set `left = seenAt + 1`.
4. Store/update `ch` → `right`, then update `maxLength` with `right - left + 1`.
5. Return `maxLength`.

**Time:** O(n), one pass. **Space:** O(min(n, alphabet size)) for the map.

### TypeScript

```typescript
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
```

### Rust

```rust
use std::collections::HashMap;

pub fn length_of_longest_substring(s: &str) -> i32 {
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut left: usize = 0;
    let mut max_length: usize = 0;

    for (right, ch) in s.chars().enumerate() {
        if let Some(&seen_at) = last_seen.get(&ch) {
            if seen_at >= left {
                left = seen_at + 1;
            }
        }
        last_seen.insert(ch, right);
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(length_of_longest_substring("bbbbb"), 1);
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
        assert_eq!(length_of_longest_substring(""), 0);
        assert_eq!(length_of_longest_substring("aab"), 2);
    }
}
```

### Notes on the two implementations

- Both store a *last-seen index* (not just a boolean set), which lets `left` jump forward in one step instead of shrinking the window character by character.
- TypeScript's `Map.get` returns `T | undefined`, so the code checks `seenAt !== undefined` before the `>=` comparison. Rust's `get` returns `Option<&usize>`, handled with `if let Some(&seen_at)`.
- The Rust version uses `s.chars().enumerate()`, counting character positions, so it works correctly on non-ASCII input too.

## Likely interviewer follow-ups

1. **Set vs map:** could you use a `Set` of characters and shrink `left` one step at a time? What changes in the time complexity?
2. **Return the substring:** how would you return the actual longest substring instead of just its length?
3. **Fixed alphabet:** if `s` were only lowercase letters, how could you replace the hash map with a fixed-size array?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Keep a window starting at `left` and ending at `right`.
2. Remember the most recent index of every character you have seen.
3. Move `right` rightward; if the new character was already seen inside the window, move `left` just past that earlier occurrence.
4. After each step, the window is duplicate-free, so measure it and keep the biggest.

## TypeScript, piece by piece

```typescript
const lastSeen = new Map<string, number>();
```
A `Map` is a key/value store. The two type arguments say the keys are strings and the values are numbers. We use it as "character → index last seen". A plain object would also work, but `Map` has a clean `.get()`/`.set()` API and is faster for many insertions.

```typescript
const ch = s.charAt(right);
```
This reads the character at index `right`. I used `charAt` instead of `s[right]` on purpose: with `noUncheckedIndexedAccess` enabled, `s[right]` is typed as `string | undefined`, which would not be accepted by `lastSeen.get`. `charAt` always returns a `string` (an empty string if the index is out of range), and here the loop guarantees `right` is in bounds.

```typescript
const seenAt = lastSeen.get(ch);
if (seenAt !== undefined && seenAt >= left) {
  left = seenAt + 1;
}
```
`lastSeen.get(ch)` returns either a number or `undefined` (key not present yet). The first condition `seenAt !== undefined` narrows the type so the second condition can safely compare numbers. The key idea is `seenAt >= left`: we only react if the previous occurrence is *inside* the current window. If it is, the window must start just after it.

```typescript
lastSeen.set(ch, right);
maxLength = Math.max(maxLength, right - left + 1);
```
`.set` records the newest index of `ch` (overwriting any older one). Then `right - left + 1` is the current window length, and `Math.max` keeps the largest seen so far.

## Rust, piece by piece

```rust
let mut last_seen: HashMap<char, usize> = HashMap::new();
```
`HashMap<char, usize>` maps characters to `usize` (the index type). `mut` is required because we insert into it later. Rust variables are immutable by default.

```rust
for (right, ch) in s.chars().enumerate() {
```
`enumerate()` pairs each item with its index, so each loop iteration destructures `(right, ch)` — the position and the character. Using `chars()` (not `bytes()`) means `right` counts characters, which is what we want for Unicode-safe indexing.

```rust
if let Some(&seen_at) = last_seen.get(&ch) {
    if seen_at >= left {
        left = seen_at + 1;
    }
}
```
`last_seen.get(&ch)` returns `Option<&usize>` — either `Some(reference)` or `None`. `if let Some(&seen_at)` matches `Some` and copies the `usize` out of the reference. The nested `if` then checks the window condition exactly like the TypeScript version.

```rust
last_seen.insert(ch, right);
max_length = max_length.max(right - left + 1);
```
`.insert` adds or overwrites the entry for `ch`. `max_length.max(other)` returns the larger of the two `usize` values and reassigns it.

```rust
max_length as i32
```
The function must return `i32`, but `right`, `left`, and `max_length` are all `usize` (Rust's unsigned index type). The final `as i32` converts the non-negative length to the requested type. Since lengths here are far below `i32::MAX`, the cast is lossless.

## Trace: `"pwwkew"` (should be `3`)

| Step | right | ch | seen_at | left | maxLength |
|---|---|---|---|---|---|
| 1 | 0 | `p` | — | 0 | 1 |
| 2 | 1 | `w` | — | 0 | 2 |
| 3 | 2 | `w` | 1 (>= 0) | 2 | 2 |
| 4 | 3 | `k` | — | 2 | 2 |
| 5 | 4 | `e` | — | 2 | 3 |
| 6 | 5 | `w` | 2 (>= 2) | 3 | 3 |

At the end `maxLength` is `3`, from the window `"wke"`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Key/value store | `Map<string, number>` | `HashMap<char, usize>` |
| Look up a key | `map.get(k)` → `T \| undefined` | `map.get(&k)` → `Option<&T>` |
| "If it exists, do X" | `if (v !== undefined)` | `if let Some(&v) = ...` |
| Insert / overwrite | `map.set(k, v)` | `map.insert(k, v)` |
| Loop with index | `for (let i = 0; ...)` | `for (i, x) in it.enumerate()` |
| Keep the max | `Math.max(a, b)` | `a.max(b)` |
| Unsigned index type | `number` | `usize` |
