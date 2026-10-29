# First Unique Character in a String (deepseek generated)

## Problem

Given a string `s` of lowercase English letters, return the **index of the first non-repeating character**, or `-1` if no such character exists. "First" means first in left-to-right order, so among all characters with a count of exactly one, pick the one that appears earliest.

## Examples

| Input | Output | Why |
|---|---|---|
| `"leetcode"` | `0` | `l` is the first character that appears once |
| `"loveleetcode"` | `2` | `v` (index 2) is the first unique character |
| `"aabb"` | `-1` | Every character repeats |
| `""` | `-1` | No characters at all |

## Constraints

- `1 <= s.length <= 10^5`
- `s` consists of lowercase English letters only.

## Things to think about before coding

- Why can't you decide on a character as "unique" while you're still reading the string for the first time?
- What do you need to know before you can scan the string a second time?
- How can you count frequencies without a hash map, given the small alphabet?

Give it a try before scrolling further.

---

## Solution

### Approach

Do two passes. The first pass counts how often each letter appears. The second pass walks the string in order and returns the first index whose letter has a count of exactly one.

1. Build a 26-slot `counts` array and increment the slot for each character.
2. Walk the string again from index 0.
3. Return the first index where `counts[letter] === 1`.
4. If none is found, return `-1`.

**Time:** O(n), two passes over the string. **Space:** O(1), the frequency table is a fixed 26 slots.

### TypeScript

```typescript
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
```

### Rust

```rust
pub fn first_uniq_char(s: &str) -> i32 {
    let mut counts = [0i32; 26];

    for b in s.bytes() {
        counts[(b - b'a') as usize] += 1;
    }

    for (i, b) in s.bytes().enumerate() {
        if counts[(b - b'a') as usize] == 1 {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(first_uniq_char("leetcode"), 0);
        assert_eq!(first_uniq_char("loveleetcode"), 2);
        assert_eq!(first_uniq_char("aabb"), -1);
        assert_eq!(first_uniq_char(""), -1);
    }
}
```

### Notes on the two implementations

- Both count first, then scan in order — the ordering matters because you must return the *first* unique character, not just any unique one.
- The return type is `number` in TypeScript but `i32` in Rust; Rust needs `i as i32` to convert the loop index from `usize` to the signed return type (which is what allows `-1`).
- The empty string naturally returns `-1` in both because the second loop never runs.

## Likely interviewer follow-ups

1. **One pass:** can you find the first unique character in a single pass, and what extra bookkeeping would it require?
2. **Unicode:** how would you adapt the counting if the string could contain any Unicode character?
3. **Index vs. character:** why is returning the *index* (not the character) important for follow-up work like substring problems?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Walk the string once and tally how many times each letter appears.
2. Walk it a second time, from left to right.
3. The first letter you meet whose tally is exactly `1` is your answer — return its position.
4. If you finish the second walk without finding one, return `-1`.

## TypeScript, piece by piece

```typescript
function firstUniqChar(s: string): number {
```
The return type `number` is used (rather than a specific index type) because the answer can be either a valid index or `-1` for "not found."

```typescript
const counts = new Array<number>(26).fill(0);
```
A 26-slot array of zeros — one slot per lowercase letter. We never reassign `counts`, so `const` is correct; we only update its contents.

```typescript
for (const ch of s) {
  counts[ch.charCodeAt(0) - 97]! += 1;
}
```
First pass: `for...of` hands us each character directly. `ch.charCodeAt(0)` converts the character to its code (`'a'` → 97), subtracting 97 maps it to a slot index 0–25. The `!` asserts the index is in bounds (it always is for `a`–`z`). Each occurrence bumps the tally by one.

```typescript
for (let i = 0; i < s.length; i++) {
  if (counts[s.charCodeAt(i) - 97]! === 1) return i;
}
```
Second pass: this time we need the *index*, so we loop with `let i`. For each character we re-derive its slot and check whether the tally is exactly `1`. The moment we find one, we return its index — left-to-right order guarantees it's the *first* unique character.

```typescript
return -1;
```
If the second pass finishes without returning, no character had a count of one, so we return the sentinel `-1`.

## Rust, piece by piece

```rust
pub fn first_uniq_char(s: &str) -> i32 {
```
The parameter is a borrowed `&str`, and the return is `i32` — a **signed** integer, because `-1` is a valid answer and unsigned types can't hold negative values.

```rust
let mut counts = [0i32; 26];
for b in s.bytes() {
    counts[(b - b'a') as usize] += 1;
}
```
First pass over raw bytes. `s.bytes()` yields each character as a `u8`. `b - b'a'` computes the offset from `'a'` (97), and `as usize` converts it to an index. Each `+= 1` tallies the letter. This is the Rust twin of `charCodeAt(i) - 97`.

```rust
for (i, b) in s.bytes().enumerate() {
```
`.enumerate()` wraps an iterator so it yields `(index, value)` pairs — here `(i, b)` where `i` is the byte position and `b` is the byte. This gives us the index we need to return, without a manual counter.

```rust
if counts[(b - b'a') as usize] == 1 {
    return i as i32;
}
```
When a letter's tally is exactly `1`, return its position. The `as i32` cast converts `i` from `usize` (unsigned) to `i32` (signed) to match the declared return type. For `a`–`z`, the byte offset is always in 0–25, so the index is safe.

```rust
-1
```
The final expression (no semicolon, no `return`) is the function's value. It's reached only when the loop never returned, meaning no character was unique.

## Trace: `"aba"` (should be `1`)

Indexes: `a`=0, `b`=1, `a`=2. Slots: index 0 = `'a'`, index 1 = `'b'`.

| Step | Phase | Value | `counts` after | Result |
|---|---|---|---|---|
| 1 | count `a` (i=0) | `a` | `[1, 0]` | — |
| 2 | count `b` (i=1) | `b` | `[1, 1]` | — |
| 3 | count `a` (i=2) | `a` | `[2, 1]` | — |
| 4 | scan i=0 | `a` → count 2, skip | — | — |
| 5 | scan i=1 | `b` → count 1 | — | return `1` |

`b` at index 1 is the first (and only) unique character.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Fixed 26 array | `new Array<number>(26).fill(0)` | `[0i32; 26]` |
| Char to index | `charCodeAt(i) - 97` | `(b - b'a') as usize` |
| Iterate with index | `for (let i = 0; i < s.length; i++)` | `for (i, b) in s.bytes().enumerate()` |
| "Not found" | return `-1` (number) | return `-1` (i32) |
| Cast index to return type | (implicit) | `i as i32` |
| Index type | `number` | `usize` (unsigned) |
| Equality | `=== 1` | `== 1` |
