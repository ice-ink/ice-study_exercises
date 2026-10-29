# Two Sum (deepseek generated)

## Problem

Given an array of integers `nums` and an integer `target`, return the **indices** of the two numbers such that they add up to `target`.

You may assume that each input has **exactly one** solution, and you may not use the same element twice. You can return the answer in any order.

## Examples

| Input | Output | Why |
|---|---|---|
| `nums = [2, 7, 11, 15]`, `target = 9` | `[0, 1]` | `2 + 7 = 9` |
| `nums = [3, 2, 4]`, `target = 6` | `[1, 2]` | `2 + 4 = 6` |
| `nums = [3, 3]`, `target = 6` | `[0, 1]` | Same value at two different indices |
| `nums = [5, 5, 5]`, `target = 10` | `[0, 1]` | Any valid pair works; don't reuse the same index |

## Constraints

- `2 <= nums.length <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
- `-10^9 <= target <= 10^9`
- Only one valid answer exists.

## Things to think about before coding

- The brute force is to check every pair — that's O(n²). Can you remember what you've already seen so you finish in one pass?
- For each number, what "complement" would make the pair sum to `target`? Where could you look that up instantly?
- What's the trade-off between the brute force and the faster approach in terms of space?

Give it a try before scrolling further.

---

## Solution

### Approach

Use a **hash map** from "value already seen" to "index where we saw it". In one pass over the array, for each number compute its complement (`target - num`). If that complement is already in the map, we've found the pair and return the two indices. Otherwise, store the current number so a later element can find it.

**Time:** O(n), one pass with O(1) map operations. **Space:** O(n) for the map.

### TypeScript

```typescript
function twoSum(nums: number[], target: number): number[] {
  const seen = new Map<number, number>(); // value -> index

  for (const [i, num] of nums.entries()) {
    const complement = target - num;
    const found = seen.get(complement);
    if (found !== undefined) {
      return [found, i];
    }
    seen.set(num, i);
  }

  return []; // unreachable: exactly one solution is guaranteed
}

// Quick checks
console.log(twoSum([2, 7, 11, 15], 9)); // [0, 1]
console.log(twoSum([3, 2, 4], 6));      // [1, 2]
console.log(twoSum([3, 3], 6));         // [0, 1]
```

### Rust

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut seen: HashMap<i32, usize> = HashMap::new(); // value -> index

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }

    vec![] // unreachable: exactly one solution is guaranteed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
```

### Notes on the two implementations

- The core idea is identical: store values we've seen and look up the complement in O(1).
- TypeScript uses `undefined` to signal "not found"; Rust uses `Option` (`None`). Both are checked before we read the value, so there's no chance of a crash.
- Rust returns indices as `i32` (via `as i32`) because the signature asks for `Vec<i32>`; internally the map uses `usize`, which is Rust's natural index type.

## Likely interviewer follow-ups

1. **Sorted input:** if the array were sorted, how would you solve it with two pointers in O(1) space?
2. **Return values instead of indices:** what changes if you must return the two *numbers* rather than their indices?
3. **Multiple solutions:** how would you return *all* pairs that sum to `target` (allowing or disallowing reuse)?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Make an empty lookup table: "value → index where I saw it".
2. For each number, ask: is `target - number` already in the table?
3. If yes, return `[index-of-that-complement, current-index]`.
4. If no, save the current number and its index, and keep going.

## TypeScript, piece by piece

```typescript
const seen = new Map<number, number>();
```
`Map<number, number>` is a dictionary where both the key and the value are numbers. Here the key is an array *value* and the value is the *index* it sits at. We choose `Map` over a plain object because keys can be any number (including negatives) and `Map.get` cleanly tells us "found vs. not found" with `undefined`.

```typescript
for (const [i, num] of nums.entries()) {
```
`.entries()` turns the array into an iterator of `[index, value]` pairs. The destructuring `const [i, num]` pulls the index into `i` and the value into `num` on each pass. (This avoids `nums[i]`, which TypeScript's strict mode types as `number | undefined` and would force extra checks.)

```typescript
const complement = target - num;
const found = seen.get(complement);
```
`complement` is the number that, added to `num`, equals `target`. `seen.get(complement)` returns the stored index if that number was seen before, or `undefined` if it wasn't.

```typescript
if (found !== undefined) {
  return [found, i];
}
```
`!==` is strict inequality. If `found` is a real index (not `undefined`), we found the pair, so we return both indices immediately and stop.

```typescript
seen.set(num, i);
```
If no match, remember this value for later. Note we store *after* checking, which also prevents accidentally reusing the same element.

## Rust, piece by piece

```rust
let mut seen: HashMap<i32, usize> = HashMap::new();
```
- `HashMap<i32, usize>` maps an `i32` value to a `usize` index.
- `mut` is required because we'll insert into it later; Rust variables are immutable by default.

```rust
for (i, &num) in nums.iter().enumerate() {
```
- `.iter()` borrows each element rather than consuming the vector, so `nums` stays usable.
- `.enumerate()` pairs each item with its index, giving `(i, &num)` where `num` is a *reference*. The `&num` pattern dereferences it into a plain `i32` copy, so arithmetic like `target - num` works directly.

```rust
if let Some(&j) = seen.get(&complement) {
    return vec![j as i32, i as i32];
}
```
This is the part worth studying:
- `seen.get(&complement)` returns an `Option<&usize>` — either `Some(&index)` or `None`.
- `if let Some(&j)` means "if it's `Some`, bind the index to `j`; otherwise skip this block." It's Rust's way of handling "maybe a value" without crashing.
- `j as i32` and `i as i32` cast the `usize` indices to `i32` to match the `Vec<i32>` return type.
- `vec![a, b]` is a shorthand for creating a vector with those two elements.

```rust
seen.insert(num, i);
```
Inserts the current value and its index so a later element can find it as a complement.

```rust
vec![]
```
The final expression with no semicolon is the function's return value. This line is unreachable in practice (the constraints guarantee a solution), but Rust requires every code path to return a `Vec<i32>`, so we provide an empty vector as a fallback.

## Trace: `nums = [2, 7, 11, 15]`, `target = 9` (should be `[0, 1]`)

| Step | i | num | complement = 9 - num | In map? | Action | Map after |
|---|---|---|---|---|---|---|
| 1 | 0 | 2 | 7 | no | store 2→0 | `{2: 0}` |
| 2 | 1 | 7 | 2 | yes (index 0) | return `[0, 1]` | — |

The algorithm never needs to see `11` or `15`, which is why it beats checking every pair.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Dictionary / map | `Map<number, number>` | `HashMap<i32, usize>` |
| Insert | `seen.set(num, i)` | `seen.insert(num, i)` |
| Lookup | `seen.get(complement)` → `number \| undefined` | `seen.get(&complement)` → `Option<&usize>` |
| "Found?" check | `found !== undefined` | `if let Some(&j) = ...` |
| Index type | `number` | `usize` |
| Build a list | `[found, i]` | `vec![j as i32, i as i32]` |
| Loop with index | `for (const [i, num] of nums.entries())` | `for (i, &num) in nums.iter().enumerate()` |
