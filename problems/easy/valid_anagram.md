# Valid Anagram (deepseek generated)

## Problem

Given two strings `s` and `t`, return `true` if `t` is an **anagram** of `s`, and `false` otherwise. An anagram contains the same letters with the same counts, just possibly in a different order. Both strings are made of lowercase English letters (`a`–`z`).

## Examples

| Input | Output | Why |
|---|---|---|
| `s = "anagram"`, `t = "nagaram"` | `true` | Same letters, same counts |
| `s = "rat"`, `t = "car"` | `false` | Different letters |
| `s = "a"`, `t = "ab"` | `false` | Different lengths |
| `s = ""`, `t = ""` | `true` | Both empty |

## Constraints

- `1 <= s.length, t.length <= 5 * 10^4`
- `s` and `t` consist of lowercase English letters only.

## Things to think about before coding

- If two strings are anagrams, what must be true about their lengths first?
- How can you tally the letters of one string and "spend" them against the other?
- Can you get away with a fixed-size table instead of a hash map, since the alphabet is only 26 letters?

Give it a try before scrolling further.

---

## Solution

### Approach

Count how many times each letter appears, using one pass over both strings together. Increment a slot for each letter in `s`, decrement for each letter in `t`, then check that every slot is back to zero.

1. If `s.length !== t.length`, return `false` (different lengths can't be anagrams).
2. Make a 26-element array `counts` of zeros, one slot per letter `'a'..'z'`.
3. For each index `i`, add 1 to the slot for `s[i]` and subtract 1 for `t[i]`.
4. If every slot is `0`, the counts matched, so return `true`; otherwise `false`.

**Time:** O(n), one pass plus a fixed 26-slot check. **Space:** O(1), the array size is a constant 26 regardless of input.

### TypeScript

```typescript
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
```

### Rust

```rust
pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut counts = [0i32; 26];

    for (sc, tc) in s.bytes().zip(t.bytes()) {
        counts[(sc - b'a') as usize] += 1;
        counts[(tc - b'a') as usize] -= 1;
    }

    counts.iter().all(|&c| c == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_anagram("anagram", "nagaram"));
        assert!(!is_anagram("rat", "car"));
        assert!(is_anagram("", ""));
        assert!(!is_anagram("a", "ab"));
    }
}
```

### Notes on the two implementations

- TypeScript indexes by `charCodeAt(i) - 97`; Rust does the same with `(b - b'a')`, where `b'a'` is the byte value 97.
- Both use a fixed 26-slot integer array, which is O(1) space — a hash map would also work but with more overhead.
- Rust pairs the two strings with `zip`, which stops at the shorter one; the length check before it makes the lengths equal.

## Likely interviewer follow-ups

1. **Unicode:** if inputs could contain `é` or emoji, why would the 26-slot array break, and what would you use instead?
2. **One counter vs. array:** why can't a single running total solve this (hint: think of `"aab"` vs `"abb"`)?
3. **Group anagrams:** how would you use this check to group a whole list of strings into anagram buckets?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. If the two strings have different lengths, they can't be anagrams — return `false`.
2. Make a 26-slot counter, one per letter, all starting at zero.
3. For each position, add 1 to the counter for the letter in `s` and subtract 1 for the letter in `t`.
4. If every counter is zero, both strings used each letter the same number of times — return `true`.

## TypeScript, piece by piece

```typescript
if (s.length !== t.length) return false;
```
A quick guard. If lengths differ, the strings can't possibly have the same letter counts, so we bail out before doing any counting. This also lets the loop below safely read both strings at the same index.

```typescript
const counts = new Array<number>(26).fill(0);
```
`new Array<number>(26)` makes an array with 26 slots (one per letter), and `.fill(0)` sets each slot to `0`. `const` is fine here because we never reassign the array itself — we only change its contents.

```typescript
for (let i = 0; i < s.length; i++) {
```
A classic index-based loop, because we need to look at `s[i]` and `t[i]` together. (`for...of` gives values but no matching index into the other string.)

```typescript
counts[s.charCodeAt(i) - 97]! += 1;
counts[t.charCodeAt(i) - 97]! -= 1;
```
`s.charCodeAt(i)` returns the numeric code for a character (`'a'` is 97, `'z'` is 122). Subtracting 97 maps `'a'`→0, `'b'`→1, …, `'z'`→25. The `!` is a **non-null assertion** needed because `noUncheckedIndexedAccess` types `counts[i]` as `number | undefined`; here the index is always in 0–25, so `!` is safe. The two lines add one count for `s` and remove one for `t`.

```typescript
return counts.every((c) => c === 0);
```
`Array.prototype.every` returns `true` only if the callback is true for *every* element. So this reads "are all 26 slots zero?" — the signature of matching counts.

## Rust, piece by piece

```rust
pub fn is_anagram(s: &str, t: &str) -> bool {
```
`&str` is a borrowed string slice — the function reads the strings without taking ownership. `-> bool` is the return type.

```rust
if s.len() != t.len() {
    return false;
}
```
`str::len()` returns the length in **bytes**. For pure lowercase ASCII, one byte equals one character, so this is the same length check as TypeScript. (For non-ASCII strings this would be a subtle bug — a good follow-up question.)

```rust
let mut counts = [0i32; 26];
```
`[0i32; 26]` is a fixed-size array of 26 `i32` integers, all initialized to zero. Unlike `Vec`, an array has a size known at compile time and lives on the stack — perfect for a small fixed table. `mut` is needed to change the slots.

```rust
for (sc, tc) in s.bytes().zip(t.bytes()) {
```
`s.bytes()` iterates the string as raw bytes (`u8`), and `.zip(...)` pairs up the bytes of `s` and `t` into `(sc, tc)` tuples. `zip` stops when either iterator ends; the earlier length check guarantees they end together.

```rust
counts[(sc - b'a') as usize] += 1;
counts[(tc - b'a') as usize] -= 1;
```
`b'a'` is the byte literal for 97. `sc - b'a'` subtracts two `u8` values to get an offset in 0–25, then `as usize` converts it to the unsigned index type arrays require. Exactly the "add for `s`, subtract for `t`" trick from the TypeScript version.

```rust
counts.iter().all(|&c| c == 0)
```
`.iter()` borrows the array so we don't move it, and `.all(...)` returns `true` only if the closure holds for every element. The closure takes `&c` (a reference), and the pattern `|&c|` dereferences it so we compare the plain `i32` value to `0`. As the last expression with no semicolon, this is the function's return value.

## Trace: `s = "ab"`, `t = "ba"` (should be `true`)

Slots: index 0 = `'a'`, index 1 = `'b'`.

| Step | `i` | `s[i]` | `t[i]` | Action | `counts` after |
|---|---|---|---|---|---|
| 0 | — | — | — | init | `[0, 0]` |
| 1 | 0 | `a` | `b` | `a`+1, `b`-1 | `[1, -1]` |
| 2 | 1 | `b` | `a` | `b`+1, `a`-1 | `[0, 0]` |
| end | — | — | — | every slot is 0 | return `true` |

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Fixed 26 array | `new Array<number>(26).fill(0)` | `[0i32; 26]` |
| Char to index | `charCodeAt(i) - 97` | `(b - b'a') as usize` |
| Length | `s.length` | `s.len()` |
| Iterate chars | `for (let i = 0; i < s.length; i++)` | `for b in s.bytes()` |
| Pair two strings | (manual index) | `s.bytes().zip(t.bytes())` |
| All-zero check | `counts.every((c) => c === 0)` | `counts.iter().all(|&c| c == 0)` |
| "Maybe undefined" | `T \| undefined` (with `!` assert) | `Option<T>` (not needed here) |
