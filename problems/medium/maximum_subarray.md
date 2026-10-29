# Maximum Subarray (deepseek generated)

## Problem

Given an integer array `nums`, return the largest sum you can obtain from any **contiguous subarray** (a run of one or more adjacent elements).

A subarray must contain at least one element, and the answer is the single number equal to that maximum sum.

## Examples

| Input | Output | Why |
|---|---|---|
| `[-2, 1, -3, 4, -1, 2, 1, -5, 4]` | `6` | The subarray `[4, -1, 2, 1]` sums to `6` |
| `[1]` | `1` | Single element is the whole array |
| `[5, 4, -1, 7, 8]` | `23` | The whole array sums to `23` |
| `[-2, -1]` | `-1` | All negative, so take the largest single element |
| `[-1]` | `-1` | One element, even though it is negative |

## Constraints

- `1 <= nums.length <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

## Things to think about before coding

- What does a "running sum" tell you, and when is a running sum dragging you down rather than helping?
- The answer can be a negative number — how does that break a naive "start at 0" approach?
- Can you do this in a single pass without a nested loop (O(n) time, O(1) space)?

Give it a try before scrolling further.

---

## Solution

### Approach

Walk the array once while keeping a **running sum**. If the running sum ever drops below `0`, it can only hurt the next subarray, so reset it to `0`. After adding each element, update the best answer seen so far.

1. Seed the answer with the first element (a valid subarray always exists).
2. Iterate every element: if the running sum is negative, reset it to `0`, then add the current element.
3. Track the maximum running sum seen.

**Time:** O(n), one pass. **Space:** O(1), a few scalar variables.

### TypeScript

```typescript
function maxSubArray(nums: number[]): number {
  let maxSoFar = nums[0]!;
  let current = 0;

  for (const n of nums) {
    if (current < 0) current = 0;
    current += n;
    if (current > maxSoFar) maxSoFar = current;
  }

  return maxSoFar;
}

// Quick checks
console.log(maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4])); // 6
console.log(maxSubArray([1]));                              // 1
console.log(maxSubArray([5, 4, -1, 7, 8]));                 // 23
console.log(maxSubArray([-2, -1]));                         // -1
```

### Rust

```rust
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_so_far = nums[0];
    let mut current = 0;

    for n in nums {
        if current < 0 {
            current = 0;
        }
        current += n;
        if current > max_so_far {
            max_so_far = current;
        }
    }

    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array(vec![-2, -1]), -1);
        assert_eq!(max_sub_array(vec![-1]), -1);
    }
}
```

### Notes on the two implementations

- The TypeScript `nums[0]!` uses a non-null assertion because `noUncheckedIndexedAccess` makes `nums[0]` look like `number | undefined`; the constraint `1 <= nums.length` guarantees it exists.
- Rust's `nums[0]` panics on an empty slice, but the problem guarantees at least one element; the function also takes `Vec<i32>` by value and consumes it in the loop.
- Both reset with an explicit `if current < 0` check, which reads more clearly than the compact `current = max(current, 0) + n` equivalent.

## Likely interviewer follow-ups

1. **All-negative input:** why must the answer be seeded with `nums[0]` instead of `0`?
2. **Return the subarray:** how would you also return the start/end indices of the best subarray?
3. **Divide and conquer:** what O(n log n) approach also works, and when might an interviewer ask for it?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Start with `maxSoFar` = the first element, and a running total `current` = `0`.
2. For each number: if `current` is negative, throw it away (set it to `0`); then add the number.
3. If the new `current` beats `maxSoFar`, remember it.
4. Return `maxSoFar` after the loop.

## TypeScript, piece by piece

```typescript
function maxSubArray(nums: number[]): number {
```
`nums: number[]` types the parameter as an array of numbers, and the trailing `: number` declares the return type. Annotating the return type is required here so callers (and `tsc`) know exactly what they get back.

```typescript
let maxSoFar = nums[0]!;
```
This reads the first element as the initial best. The `!` is a **non-null assertion**: with `noUncheckedIndexedAccess` enabled, `nums[0]` is typed `number | undefined`, and `!` tells TypeScript "I know this index is safe." It is safe because the constraints guarantee `nums.length >= 1`.

```typescript
let current = 0;
```
The running sum starts at `0`. It is `let` (not `const`) because it gets reassigned on every iteration.

```typescript
if (current < 0) current = 0;
current += n;
```
If the running sum is negative, it can only drag down any future sum, so we reset it. Then we add the current element. `current += n` is shorthand for `current = current + n`.

```typescript
if (current > maxSoFar) maxSoFar = current;
```
After every addition we ask "is this the best sum so far?" and update the answer if so. Because the answer is a `let`, we keep mutating the same variable rather than building a list.

## Rust, piece by piece

```rust
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
```
`pub` makes the function visible outside its module. `Vec<i32>` is an owned vector of 32-bit integers; the function **takes ownership** of it (no `&`), which is fine because we never need it again after this call.

```rust
let mut max_so_far = nums[0];
```
`nums[0]` indexes the vector and returns an `i32` (which is `Copy`, so it is copied out). `mut` is required because Rust variables are immutable by default and we reassign `max_so_far` later.

```rust
for n in nums {
```
This loop **moves** `nums`, so it can no longer be used afterward. Each `n` is an owned `i32`. If you needed `nums` after the loop, you'd write `for &n in &nums` or `for n in nums.iter()` instead.

```rust
if current < 0 {
    current = 0;
}
```
Rust `if` bodies need braces, unlike the single-statement TypeScript version. This is the same "reset a negative running sum" logic.

```rust
current += n;
if current > max_so_far {
    max_so_far = current;
}
max_so_far
```
The last expression has no semicolon and no `return`, so it is the function's return value. Adding a semicolon would turn it into a statement returning `()`, which would not match `-> i32` and would fail to compile.

## Trace: `[-2, 1, -3, 4]` (should be `4`)

| Step | n | current after reset + add | maxSoFar |
|---|---|---|---|
| init | — | 0 | -2 |
| 1 | -2 | -2 | -2 |
| 2 | 1 | 1 (reset to 0 first, then +1) | 1 |
| 3 | -3 | -2 | 1 |
| 4 | 4 | 4 (reset to 0 first, then +4) | 4 |

After step 4 the best sum is `4`, which comes from the single-element subarray `[4]`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Non-empty seed | `nums[0]!` (non-null assertion) | `nums[0]` (plain index) |
| Mutable variable | `let` | `let mut` |
| Negative-reset branch | `if (current < 0) current = 0;` | `if current < 0 { current = 0; }` |
| Add current element | `current += n;` | `current += n;` |
| Function return | `return maxSoFar;` | `maxSoFar` (implicit, no `;`) |
| Array parameter | `nums: number[]` | `nums: Vec<i32>` |
