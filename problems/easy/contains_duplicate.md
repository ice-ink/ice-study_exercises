# Contains Duplicate (deepseek generated)

## Problem

Given an integer array `nums`, return `true` if any value appears **at least twice** in the array. Return `false` if every element is distinct.

## Examples

| Input | Output | Why |
|---|---|---|
| `[1, 2, 3, 1]` | `true` | `1` appears twice |
| `[1, 2, 3, 4]` | `false` | All elements distinct |
| `[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]` | `true` | Many repeats |
| `[]` | `false` | Empty array has no duplicates |

## Constraints

- `0 <= nums.length <= 10^5`
- `-10^9 <= nums[i] <= 10^9`

## Things to think about before coding

- The brute force compares every pair — how slow is that?
- What data structure remembers "have I seen this value before" in O(1) time?
- Is there a way to stop early the moment you find the first duplicate?

Give it a try before scrolling further.

---

## Solution

### Approach

Walk the array once while keeping a **set** of values already seen. For each number, if it's already in the set, we've found a duplicate and can return `true` immediately; otherwise add it and continue.

1. Create an empty set.
2. For each `n` in `nums`, check whether `n` is already in the set.
3. If it is, return `true`. Otherwise insert `n`.
4. If the loop finishes without a match, return `false`.

**Time:** O(n), one pass with O(1) average set operations. **Space:** O(n), the set holds up to n values.

### TypeScript

```typescript
function containsDuplicate(nums: number[]): boolean {
  const seen = new Set<number>();

  for (const n of nums) {
    if (seen.has(n)) return true;
    seen.add(n);
  }

  return false;
}

// Quick checks
console.log(containsDuplicate([1, 2, 3, 1])); // true
console.log(containsDuplicate([1, 2, 3, 4])); // false
console.log(containsDuplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])); // true
console.log(containsDuplicate([])); // false
```

### Rust

```rust
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for n in nums {
        if !seen.insert(n) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(!contains_duplicate(vec![]));
    }
}
```

### Notes on the two implementations

- TypeScript separates the check and the insert (`has` then `add`), which is clear but does two lookups in the worst case.
- Rust's `HashSet::insert` returns a `bool` — `false` if the value was already present — so a single call both checks and inserts.
- Both return early, so an early duplicate avoids scanning the rest of the array.

## Likely interviewer follow-ups

1. **Two lookups:** in TypeScript, how could you do the check-and-insert in one operation, and why doesn't it change the complexity?
2. **Sorting alternative:** how would you solve it by sorting first, and what's the time/space tradeoff?
3. **Memory bound:** if you could only use O(1) extra space, what would the best time become?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Make an empty set to remember numbers you've already seen.
2. Look at each number in turn.
3. If the current number is already in the set, you found a duplicate — return `true`.
4. Otherwise, add it to the set and keep going.
5. If you finish the whole array without a repeat, return `false`.

## TypeScript, piece by piece

```typescript
function containsDuplicate(nums: number[]): boolean {
```
The parameter is an array of numbers, and the return type `boolean` says the answer is `true` or `false`.

```typescript
const seen = new Set<number>();
```
`Set<number>` is a collection that stores each value only once. The `new` keyword constructs an empty one. Sets are the right tool here because `has` and `add` are both O(1) on average, so checking "have I seen this?" is fast.

```typescript
for (const n of nums) {
```
`for...of` loops over the *values* of the array, assigning each to `n`. (Don't confuse it with `for...in`, which gives you index keys.)

```typescript
if (seen.has(n)) return true;
seen.add(n);
```
`seen.has(n)` asks "is `n` in the set?" If yes, we immediately `return true` — no need to look at anything else. If no, `seen.add(n)` records it for future checks. Notice the function stops early the instant it finds a duplicate.

```typescript
return false;
```
Reaching this line means the loop completed without ever finding a match, so every element was distinct.

## Rust, piece by piece

```rust
use std::collections::HashSet;
```
`HashSet` lives in the standard library's `collections` module, so it must be imported with `use` before we can name it. This is Rust's equivalent of an import.

```rust
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
```
The function takes ownership of a `Vec<i32>` (a growable array of 32-bit signed integers) and returns a `bool`. Unlike TypeScript's `number`, Rust's `i32` has a fixed size and range.

```rust
let mut seen = HashSet::new();
```
`HashSet::new()` creates an empty set. `mut` is required because we insert into it later — Rust variables are immutable by default, and `insert` needs to change the set.

```rust
for n in nums {
    if !seen.insert(n) {
        return true;
    }
}
```
This is the key line: `seen.insert(n)` adds `n` and returns a `bool` — `true` if it *was newly inserted*, `false` if `n` was already present. So `!seen.insert(n)` (the `!` negates) is `true` exactly when `n` was a duplicate, and we return `true`. One call does both the check and the insert.

```rust
false
```
The final expression with no semicolon and no `return` is the function's return value. It runs only if the loop never found a duplicate.

## Trace: `[1, 2, 3, 1]` (should be `true`)

| Step | `n` | In `seen`? | `seen` after | Return? |
|---|---|---|---|---|
| 1 | 1 | no | `{1}` | — |
| 2 | 2 | no | `{1, 2}` | — |
| 3 | 3 | no | `{1, 2, 3}` | — |
| 4 | 1 | yes | — | `true` |

The second `1` is already in the set, so we return `true` without doing anything else.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Set type | `Set<number>` | `HashSet<i32>` |
| Create empty set | `new Set<number>()` | `HashSet::new()` |
| Check membership | `seen.has(n)` | `seen.contains(&n)` |
| Add value | `seen.add(n)` | `seen.insert(n)` |
| Insert returns whether new | (no) | `true`/`false` |
| Loop over values | `for (const n of nums)` | `for n in nums` |
| Import | (global) | `use std::collections::HashSet;` |
