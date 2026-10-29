# Top K Frequent Elements (deepseek generated)

## Problem

Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. The answer is guaranteed to be unique, but you may return it in any order.

## Examples

| Input | Output | Why |
|---|---|---|
| `nums = [1,1,1,2,2,3], k = 2` | `[1,2]` | `1` appears 3 times, `2` appears 2 times |
| `nums = [1], k = 1` | `[1]` | Only one element |
| `nums = [4,4,4,2,2,2,1,1,3], k = 3` | `[4,2,1]` | Frequencies: `4`→3, `2`→3, `1`→2, `3`→1 |
| `nums = [1,1,2,2], k = 2` | `[1,2]` | Tie in frequency; any order is fine |

## Constraints

- `1 <= nums.length <= 10^5`
- `-10^4 <= nums[i] <= 10^4`
- `k` is in the range `[1, number of unique elements]`.
- The answer is guaranteed to be unique (no frequency ties at the `k` boundary).

## Things to think about before coding

- How do you count each element's frequency efficiently?
- A **min-heap** of size `k` works in O(n log k). Can you do better by noticing the maximum possible frequency is `n`?
- If you put elements into buckets indexed by *count*, scanning buckets from high to low gives the top `k` in linear time.
- The `k` boundary is guaranteed unambiguous, so you don't need to worry about tie-breaking.

Give it a try before scrolling further.

---

## Solution

### Approach

Count frequencies with a hash map, then use **bucket sort by frequency**. The maximum count any element can have is `nums.length`, so create that many buckets, where `bucket[c]` lists every number that appears exactly `c` times. Scanning buckets from highest count down yields the `k` most frequent elements.

1. Build a frequency map from number → count.
2. Create `nums.length + 1` empty buckets.
3. Put each number into the bucket for its count.
4. Walk the buckets from highest index down, appending numbers until `k` are collected.
5. Return the collected numbers.

**Time:** O(n), counting plus one bucket scan. **Space:** O(n) for the map and buckets.

### TypeScript

```typescript
function topKFrequent(nums: number[], k: number): number[] {
  const freq = new Map<number, number>();
  for (const num of nums) {
    freq.set(num, (freq.get(num) ?? 0) + 1);
  }

  // buckets[i] holds every number that appears exactly i times.
  const buckets: number[][] = Array.from({ length: nums.length + 1 }, () => []);
  for (const [num, count] of freq) {
    buckets[count]!.push(num);
  }

  const result: number[] = [];
  for (let i = buckets.length - 1; i >= 0 && result.length < k; i--) {
    const bucket = buckets[i]!;
    for (const num of bucket) {
      result.push(num);
      if (result.length === k) break;
    }
  }

  return result;
}

// Quick checks
console.log(topKFrequent([1, 1, 1, 2, 2, 3], 2));        // [1, 2]
console.log(topKFrequent([1], 1));                        // [1]
console.log(topKFrequent([4, 4, 4, 2, 2, 2, 1, 1, 3], 3)); // [4, 2, 1]
```

### Rust

```rust
use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        *freq.entry(num).or_default() += 1;
    }

    // buckets[count] holds every number that appears exactly `count` times.
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
    for (&num, &count) in &freq {
        buckets[count].push(num);
    }

    let mut result: Vec<i32> = Vec::with_capacity(k as usize);
    for i in (0..buckets.len()).rev() {
        for &num in &buckets[i] {
            result.push(num);
            if result.len() == k as usize {
                return result;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
```

### Notes on the two implementations

- Both use bucket sort, which is O(n) instead of the O(n log k) heap approach; the bucket array size is `n + 1` because counts range from `0` to `n`.
- TypeScript needs a non-null assertion (`buckets[count]!`) because `noUncheckedIndexedAccess` types every bucket read as possibly `undefined`; the count is always in range.
- Rust stores counts as `usize` (needed for array indexing) but the function takes/returns `i32`/`Vec<i32>`, so `k` is cast with `k as usize` for the capacity check.

## Likely interviewer follow-ups

1. **Heap version:** how would you solve it with a min-heap of size `k`? Compare its time and space.
2. **Stable order:** if you had to return the result sorted by frequency then value, what would change?
3. **Streaming:** what if `nums` arrives as a stream and you must keep the top `k` at all times?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Count how many times each number appears.
2. Make an array of buckets where the index equals a frequency.
3. Drop each number into the bucket for its frequency.
4. Sweep buckets from largest frequency to smallest, collecting numbers until you have `k`.

## TypeScript, piece by piece

```typescript
freq.set(num, (freq.get(num) ?? 0) + 1);
```
`freq.get(num)` returns the current count or `undefined`. The **nullish coalescing operator** `??` yields its right-hand side only when the left side is `null`/`undefined`, so an unseen number starts at `0` and becomes `1`.

```typescript
const buckets: number[][] = Array.from({ length: nums.length + 1 }, () => []);
```
`Array.from({ length: n }, mapFn)` builds an array of `n` items, calling `() => []` for each — this creates a *fresh* empty array per bucket. (Using `new Array(n).fill([])` would be a bug: every bucket would be the *same* array.)

```typescript
for (const [num, count] of freq) {
  buckets[count]!.push(num);
}
```
Iterating a `Map` yields `[key, value]` pairs, so `num` is the element and `count` its frequency. `count` is always between `1` and `nums.length`, so the bucket exists; the `!` tells TypeScript "trust me, this isn't `undefined`".

```typescript
for (let i = buckets.length - 1; i >= 0 && result.length < k; i--) {
  const bucket = buckets[i]!;
  for (const num of bucket) {
    result.push(num);
    if (result.length === k) break;
  }
}
```
Start at the highest frequency and walk down. The loop condition `result.length < k` stops the outer loop as soon as we have enough, and the inner `break` exits a bucket early so we never collect more than `k`.

## Rust, piece by piece

```rust
for &num in &nums {
    *freq.entry(num).or_default() += 1;
}
```
`&nums` iterates by reference, and `&num` destructures the `&i32` back to a plain `i32`. `entry(num).or_default()` returns a `&mut usize` (creating `0` if absent), and the leading `*` dereferences it so `+= 1` increments the stored count.

```rust
let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
```
`vec![value; n]` makes a `Vec` of `n` copies of `value`. Here `Vec::new()` is called once per slot, so each bucket is a distinct, independent empty vector. The size `n + 1` covers counts `0..=n`.

```rust
for (&num, &count) in &freq {
    buckets[count].push(num);
}
```
Iterating `&freq` yields `(&i32, &usize)`. The pattern `(&num, &count)` copies the values out of the references, leaving `num: i32` and `count: usize`. `count` is a valid bucket index, so no `Option` handling is needed.

```rust
for i in (0..buckets.len()).rev() {
    for &num in &buckets[i] {
        result.push(num);
        if result.len() == k as usize {
            return result;
        }
    }
}
```
`(0..n).rev()` walks the range backward (Rust has no `i--`). `k` is `i32` but `result.len()` is `usize`, so `k as usize` makes the comparison type-correct. Returning early inside the loops is idiomatic Rust.

```rust
let mut result: Vec<i32> = Vec::with_capacity(k as usize);
```
`with_capacity` pre-allocates space for `k` elements, avoiding repeated reallocation as we push. This is a minor, idiomatic optimization.

## Trace: `nums = [1,1,1,2,2,3], k = 2` (should be `[1,2]`)

`freq = {1: 3, 2: 2, 3: 1}`; buckets sized `7`: `bucket[1] = [3]`, `bucket[2] = [2]`, `bucket[3] = [1]`.

| Step | i (frequency) | bucket[i] | result after |
|---|---|---|---|
| 1 | 6 → 4 | empty | `[]` |
| 2 | 3 | `[1]` | `[1]` |
| 3 | 2 | `[2]` | `[1, 2]` — reached `k`, stop |

The answer is `[1, 2]`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Frequency map | `Map<number, number>` | `HashMap<i32, usize>` |
| Increment a count | `(freq.get(x) ?? 0) + 1` | `*freq.entry(x).or_default() += 1` |
| Array of N empty arrays | `Array.from({length:n}, () => [])` | `vec![Vec::new(); n]` |
| Iterate map entries | `for (const [k, v] of m)` | `for (&k, &v) in &m` |
| Reverse loop | `for (let i = n-1; i >= 0; i--)` | `for i in (0..n).rev()` |
| Non-null indexed access | `arr[i]!` | plain `arr[i]` (panics if OOB) |
| Return early | `return result;` | `return result;` (same) |
