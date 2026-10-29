# Three Sum (deepseek generated)

## Problem

Given an integer array `nums`, return every **unique** triplet `[nums[i], nums[j], nums[k]]` such that `i`, `j`, and `k` are distinct indices and the three values sum to `0`.

The solution set must not contain duplicate triplets; the order of the triplets (and of values within each triplet) does not matter.

## Examples

| Input | Output | Why |
|---|---|---|
| `[-1, 0, 1, 2, -1, -4]` | `[[-1, -1, 2], [-1, 0, 1]]` | Two distinct triplets sum to 0 |
| `[0, 1, 1]` | `[]` | No triplet sums to 0 |
| `[0, 0, 0]` | `[[0, 0, 0]]` | The only triplet, used once |
| `[-2, 0, 0, 2, 2]` | `[[-2, 0, 2]]` | Duplicates must not be repeated |

## Constraints

- `0 <= nums.length <= 3000`
- `-10^5 <= nums[i] <= 10^5`

## Things to think about before coding

- A brute-force triple loop is O(n³) — what does sorting buy you?
- Once you fix one number, the problem becomes "two sum" on the rest. How do you find two numbers that sum to a target in a sorted array?
- Duplicate triplets are the tricky part: where can duplicates sneak in, and how do you skip them at the right moments?

Give it a try before scrolling further.

---

## Solution

### Approach

Sort the array, then fix each element as the first value of the triplet and run a **two-pointer** search on the rest of the array for a pair that sums to `-nums[i]`. Skipping duplicate pivots and duplicate pair values guarantees unique triplets.

1. Sort a copy of the array.
2. For each index `i` (skipping a pivot equal to the previous one), set `left = i + 1` and `right = n - 1`.
3. Sum the three; if too small move `left` right, if too big move `right` left.
4. On a hit, record the triplet and move both pointers past any duplicates.

**Time:** O(n²) — O(n log n) sort plus O(n²) two-pointer scan. **Space:** O(1) excluding the output (the sorted copy is O(n) in TypeScript; Rust reuses the input).

### TypeScript

```typescript
function threeSum(nums: number[]): number[][] {
  const sorted = [...nums].sort((a, b) => a - b);
  const result: number[][] = [];

  for (let i = 0; i < sorted.length - 2; i++) {
    if (i > 0 && sorted[i] === sorted[i - 1]) continue;
    let left = i + 1;
    let right = sorted.length - 1;

    while (left < right) {
      const sum = sorted[i]! + sorted[left]! + sorted[right]!;
      if (sum < 0) {
        left++;
      } else if (sum > 0) {
        right--;
      } else {
        result.push([sorted[i]!, sorted[left]!, sorted[right]!]);
        left++;
        right--;
        while (left < right && sorted[left] === sorted[left - 1]) left++;
        while (left < right && sorted[right] === sorted[right + 1]) right--;
      }
    }
  }

  return result;
}

// Quick checks
console.log(JSON.stringify(threeSum([-1, 0, 1, 2, -1, -4]))); // [[-1,-1,2],[-1,0,1]]
console.log(JSON.stringify(threeSum([0, 1, 1])));             // []
console.log(JSON.stringify(threeSum([0, 0, 0])));             // [[0,0,0]]
console.log(JSON.stringify(threeSum([-2, 0, 0, 2, 2])));      // [[-2,0,2]]
```

### Rust

```rust
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut result: Vec<Vec<i32>> = Vec::new();

    let n = nums.len();
    for i in 0..n.saturating_sub(2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum < 0 {
                left += 1;
            } else if sum > 0 {
                right -= 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
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
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(three_sum(vec![-2, 0, 0, 2, 2]), vec![vec![-2, 0, 2]]);
    }
}
```

### Notes on the two implementations

- TypeScript copies the input with `[...nums]` before sorting to avoid mutating the caller; Rust takes ownership and uses `sort_unstable`, which is fine (and faster) for integers where stability doesn't matter.
- Both rely on the sorted order to know which pointer to move: a too-small sum means `left` must grow, a too-large sum means `right` must shrink.
- In Rust, `n.saturating_sub(2)` avoids underflow on the empty/short-array cases, since `n` is a `usize` and plain `n - 2` would panic for `n < 2`.

## Likely interviewer follow-ups

1. **Return indices instead:** how does the problem change if you must return original indices (hint: sorting loses them)?
2. **Target ≠ 0:** how would you generalize to "find triples summing to `target`"?
3. **Lower bound:** can this be done faster than O(n²), and what is the theoretical lower bound?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Sort the numbers.
2. For each index, treat its value as the fixed first number.
3. Use two pointers (`left` and `right`) on the remaining numbers to find a pair that sums to the negation of the fixed number.
4. Move the pointers inward based on whether the sum is too small or too big, and skip duplicates everywhere.

## TypeScript, piece by piece

```typescript
const sorted = [...nums].sort((a, b) => a - b);
```
We copy the array (`[...nums]`) so the caller's array isn't reordered, then sort ascending with a numeric comparator. Sorting is what lets the two-pointer step make a single, correct decision at each step.

```typescript
for (let i = 0; i < sorted.length - 2; i++) {
```
The loop stops at `length - 2` because a triplet needs at least three remaining values (`i`, `left`, `right`).

```typescript
if (i > 0 && sorted[i] === sorted[i - 1]) continue;
```
This skips a pivot equal to the previous one. If we didn't, the same first value would produce duplicate triplets. `continue` jumps straight to the next iteration of the loop.

```typescript
const sum = sorted[i]! + sorted[left]! + sorted[right]!;
```
The `!` assertions mark the three index reads as safe (the pointers are always in bounds). Because the array is sorted, `sum < 0` means we need a larger value, so `left++`; `sum > 0` means we need a smaller value, so `right--`.

```typescript
result.push([sorted[i]!, sorted[left]!, sorted[right]!]);
left++;
right--;
while (left < right && sorted[left] === sorted[left - 1]) left++;
while (left < right && sorted[right] === sorted[right + 1]) right--;
```
On a hit we record the triplet, then move both pointers inward. The two `while` loops skip over any identical neighbors so the same pair isn't recorded twice.

## Rust, piece by piece

```rust
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
```
The function takes ownership of the input vector and returns a vector of vectors. Because we own `nums`, we can sort it in place without copying.

```rust
let mut nums = nums;
nums.sort_unstable();
```
Rebinding the parameter as `mut` lets us sort it (parameters are immutable by default). `sort_unstable` sorts in place; "unstable" means it may reorder equal elements, which is fine here since equal values are interchangeable.

```rust
let n = nums.len();
for i in 0..n.saturating_sub(2) {
```
`saturating_sub(2)` returns `n - 2`, but clamps to `0` instead of underflowing when `n` is `0` or `1`. That makes the range empty for tiny arrays instead of panicking.

```rust
if i > 0 && nums[i] == nums[i - 1] {
    continue;
}
```
Same duplicate-pivot skip as TypeScript. Note `nums[i] == nums[i - 1]` is only reached when `i > 0`, so the index `i - 1` is always valid.

```rust
let sum = nums[i] + nums[left] + nums[right];
if sum < 0 {
    left += 1;
} else if sum > 0 {
    right -= 1;
} else {
    result.push(vec![nums[i], nums[left], nums[right]]);
    ...
}
```
`vec![...]` builds a new `Vec<i32>` for the triplet and `push` moves it into `result`. The `if`/`else if`/`else` chain mirrors the three-way sum comparison.

```rust
while left < right && nums[left] == nums[left - 1] {
    left += 1;
}
while left < right && nums[right] == nums[right + 1] {
    right -= 1;
}
```
These skip duplicate pair values after a match. The `left < right` condition is re-checked each iteration so the pointers never cross or index out of bounds.

## Trace: `[-1, 0, 1, 2, -1, -4]` (should be `[[-1, -1, 2], [-1, 0, 1]]`)

Sorted input: `[-4, -1, -1, 0, 1, 2]`.

| Step | i (pivot) | left / right | Sum | Action | result |
|---|---|---|---|---|---|
| 1 | 0 (`-4`) | 1 / 5 | `-4 + -1 + 2 = -3` | too small, `left++` | `[]` |
| 2 | 0 (`-4`) | 2 / 5 | `-4 + -1 + 2 = -3` | too small, `left++` | `[]` |
| 3 | 1 (`-1`) | 2 / 5 | `-1 + -1 + 2 = 0` | match `[-1,-1,2]`, skip dups | `[[-1,-1,2]]` |
| 4 | 1 (`-1`) | 3 / 4 | `-1 + 0 + 1 = 0` | match `[-1,0,1]`, pointers cross | `[[-1,-1,2],[-1,0,1]]` |
| 5 | 2 (`-1`) | — | — | duplicate pivot, skip | (unchanged) |
| 6 | 3 (`0`) | 4 / 5 | `0 + 1 + 2 = 3` | too big, `right--`, then cross | (unchanged) |

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Sort a copy / in place | `[...nums].sort((a, b) => a - b)` | `nums.sort_unstable()` |
| Loop with early exit | `continue` | `continue` |
| Underflow-safe bound | `i < sorted.length - 2` | `0..n.saturating_sub(2)` |
| Read a value | `sorted[i]!` | `nums[i]` |
| Build a triplet | `[a, b, c]` | `vec![a, b, c]` |
| Move pointer | `left++` / `right--` | `left += 1` / `right -= 1` |
