# Sliding Window Maximum (deepseek generated)

## Problem

Given an array of integers `nums` and a window size `k`, return an array where each element is the maximum value of the sliding window of size `k` as the window moves one position at a time from left to right. There are exactly `nums.length - k + 1` windows.

## Examples

| Input | Output | Why |
|---|---|---|
| `nums = [1,3,-1,-3,5,3,6,7]`, `k = 3` | `[3,3,5,5,6,7]` | Max of each size-3 window in order |
| `nums = [1]`, `k = 1` | `[1]` | Single element is its own max |
| `nums = [7,2,4]`, `k = 2` | `[7,4]` | Max of `[7,2]` then `[2,4]` |
| `nums = [1,-1]`, `k = 1` | `[1,-1]` | With `k = 1` every element is a window |
| `nums = [4,3,2,1]`, `k = 2` | `[4,3,2]` | Strictly decreasing input, no eviction needed |

## Constraints

- `1 <= nums.length <= 10^5`
- `-10^4 <= nums[i] <= 10^4`
- `1 <= k <= nums.length`

## Things to think about before coding

- A naive "rescan each window for its max" does O(n·k) work. What structure could let you read the max in O(1)?
- If you encounter a value bigger than some earlier value, can that earlier value ever be the max of a *future* window? Why or why not?
- Elements also "expire" as the window slides. How do you know when the current max has left the window?
- Target O(n) time and O(k) space.

Give it a try before scrolling further.

---

## Solution

### Approach

Keep a **monotonic deque of indices** whose values are strictly decreasing. The front of the deque is always the current window's maximum. For each new element:

1. Drop indices from the front that have slid out of the window (index `< i - k + 1`).
2. Pop indices from the back while their value is `<=` the incoming value — they can never be the max again.
3. Push the new index onto the back.
4. Once the first window is full (`i >= k - 1`), the front index gives the max.

**Time:** O(n), each index is pushed and popped at most once. **Space:** O(k) for the deque.

### TypeScript

```typescript
function maxSlidingWindow(nums: number[], k: number): number[] {
  const n = nums.length;
  const result: number[] = new Array<number>(n - k + 1);
  // deque holds indices; the values at those indices are strictly decreasing.
  const deque: number[] = [];

  for (let i = 0; i < n; i++) {
    // Drop indices that have slid out of the window's left edge.
    if (deque.length > 0 && deque[0]! < i - k + 1) {
      deque.shift();
    }

    // Pop from the back while the incoming value is >= the back's value.
    while (deque.length > 0 && nums[deque[deque.length - 1]!]! <= nums[i]!) {
      deque.pop();
    }

    deque.push(i);

    // Once the first window is full, the front of the deque is the maximum.
    if (i >= k - 1) {
      result[i - k + 1] = nums[deque[0]!]!;
    }
  }

  return result;
}

// Quick checks
console.log(maxSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3)); // [3,3,5,5,6,7]
console.log(maxSlidingWindow([1], 1)); // [1]
console.log(maxSlidingWindow([1, -1], 1)); // [1,-1]
console.log(maxSlidingWindow([7, 2, 4], 2)); // [7,4]
```

### Rust

```rust
use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if k == 0 || k > n {
        return Vec::new();
    }

    let mut result: Vec<i32> = Vec::with_capacity(n - k + 1);
    // deque holds indices; the values at those indices are strictly decreasing.
    let mut deque: VecDeque<usize> = VecDeque::new();

    for (i, &value) in nums.iter().enumerate() {
        // Drop indices that have slid out of the window's left edge.
        if let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            }
        }

        // Pop from the back while the incoming value is >= the back's value.
        while let Some(&back) = deque.back() {
            if nums[back] <= value {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        // Once the first window is full, the front of the deque is the maximum.
        if i + 1 >= k {
            result.push(nums[deque[0]]);
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
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
        assert_eq!(max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
    }

    #[test]
    fn empty_when_k_too_large() {
        assert!(max_sliding_window(vec![1, 2], 3).is_empty());
    }
}
```

### Notes on the two implementations

- Both store **indices**, not values, so we can tell whether an entry has left the window. A value alone cannot tell us if it has expired.
- TypeScript uses `shift()`/`pop()` on a plain array, which is easy to read; Rust's `VecDeque` gives true O(1) `pop_front()`/`pop_back()`.
- The Rust version guards `k == 0 || k > n` up front so the `n - k + 1` capacity calculation can never underflow.

## Likely interviewer follow-ups

1. **True O(n) in JS:** `Array.prototype.shift()` is O(k); how would you get a strict O(n) using a circular buffer or head pointer instead?
2. **Min instead of max:** what single comparison changes if you need the sliding-window *minimum*?
3. **2D version:** how would you extend this to the max of every `k × k` sub-matrix?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Make an empty deque that will hold **array indices**, not values.
2. For each index `i` (left to right):
   - Remove the front index if it is no longer inside the window.
   - From the back, remove every index whose value is `<= nums[i]`.
   - Push `i` onto the back.
3. When the window is full, the value at the front index is that window's maximum.

## TypeScript, piece by piece

```typescript
const deque: number[] = [];
```
We store indices in a plain array and use it as a deque: `push` adds to the back, `pop` removes from the back, and `shift` removes from the front.

```typescript
if (deque.length > 0 && deque[0]! < i - k + 1) {
  deque.shift();
}
```
The window ending at `i` starts at index `i - k + 1`. If the front index is smaller than that, it has left the window. `deque[0]!` is a non-null assertion: because the `length > 0` check guarantees the slot exists, `!` tells TypeScript to drop the `undefined` that `noUncheckedIndexedAccess` would otherwise add.

```typescript
while (deque.length > 0 && nums[deque[deque.length - 1]!]! <= nums[i]!) {
  deque.pop();
}
```
Any value smaller than or equal to the incoming value can never be the maximum again — the incoming value is bigger and stays in the window longer (it has a larger index). So we pop them from the back until the deque is empty or the back value is strictly larger. The double `!` asserts both the index slot and the array lookup are defined.

```typescript
deque.push(i);

if (i >= k - 1) {
  result[i - k + 1] = nums[deque[0]!]!;
}
```
We always add the current index. Before the window reaches size `k` there is no result to record; from `i = k - 1` onward, the front of the deque is guaranteed to be the window's maximum, so we read `nums[deque[0]]` into the correct result slot.

## Rust, piece by piece

```rust
use std::collections::VecDeque;
let mut deque: VecDeque<usize> = VecDeque::new();
```
`VecDeque` is Rust's double-ended queue. Unlike a plain `Vec`, it supports O(1) `pop_front()` and `push_front()` in addition to the back operations, so we do not have to shift every element like TypeScript's `shift()`.

```rust
for (i, &value) in nums.iter().enumerate() {
```
`enumerate()` pairs each element with its index. The pattern `&value` destructures the `&i32` reference the iterator yields, giving us a plain `i32` copy. `i` is a `usize`, the correct type for array indices.

```rust
if let Some(&front) = deque.front() {
    if front + k <= i {
        deque.pop_front();
    }
}
```
`deque.front()` returns `Option<&usize>` — `None` when empty. `if let` binds only when there *is* a front, which is how Rust handles "maybe there is a value" without crashing. `front + k <= i` is the same "expired" test as `front < i - k + 1`, written without subtraction.

```rust
while let Some(&back) = deque.back() {
    if nums[back] <= value {
        deque.pop_back();
    } else {
        break;
    }
}
```
`deque.back()` gives the back index or `None`. We pop while the back's value is `<= value`; the `else { break }` is required because, unlike the TypeScript `while` condition, we cannot re-test the value inside the condition after binding it.

## Trace: `nums = [1,3,-1,-3,5,3]`, `k = 3` (should be `[3,3,5,5]`)

| Step | i | nums[i] | Action | deque (indices) after |
|---|---|---|---|---|
| 1 | 0 | 1 | push 0 | `[0]` |
| 2 | 1 | 3 | pop 0 (1 ≤ 3), push 1 | `[1]` |
| 3 | 2 | -1 | push 2 | `[1,2]` |
| 4 | 3 | -3 | push 3 | `[1,2,3]` |
| 5 | 4 | 5 | drop 1 (expired), pop 3, pop 2, push 4 | `[4]` |
| 6 | 5 | 3 | push 5 | `[4,5]` |

Recording the front value once the window is full gives: `i=2 → 3`, `i=3 → 3`, `i=4 → 5`, `i=5 → 5`, i.e. `[3,3,5,5]`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Double-ended queue | plain array + `push`/`pop`/`shift` | `VecDeque` with `push_back`/`pop_front` |
| Loop with index and value | `for (let i = 0; i < n; i++)` | `for (i, &value) in nums.iter().enumerate()` |
| "Maybe empty" front | `length > 0` guard | `Option<&usize>` via `.front()` |
| Indexing with safe guard | non-null assertion `deque[0]!` | no `!` needed; indices are always valid `usize` |
| Assert a value is present | `x!` | pattern matching / `if let` |
| Empty result for bad input | implicit (loops just don't run) | explicit `return Vec::new()` |
