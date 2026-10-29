# Merge Intervals (deepseek generated)

## Problem

Given an array of intervals where each interval is `[start, end]`, merge all intervals that overlap and return a list of the non-overlapping intervals that cover all the ranges in the input.

Two intervals overlap when one starts before the other ends (touching endpoints count as overlapping). The result may be returned in any order, but the standard approach returns them sorted by start.

## Examples

| Input | Output | Why |
|---|---|---|
| `[[1, 3], [2, 6], [8, 10], [15, 18]]` | `[[1, 6], [8, 10], [15, 18]]` | `[1,3]` and `[2,6]` overlap |
| `[[1, 4], [4, 5]]` | `[[1, 5]]` | Touching at `4` counts as overlap |
| `[[1, 4], [0, 4]]` | `[[0, 4]]` | Second starts earlier and extends to `4` |
| `[[1, 4], [5, 6]]` | `[[1, 4], [5, 6]]` | No overlap, keep both |
| `[]` | `[]` | Empty input gives empty output |

## Constraints

- `0 <= intervals.length <= 10^4`
- `intervals[i].length == 2`
- `0 <= start <= end <= 10^4`

## Things to think about before coding

- Why does sorting by start make the problem dramatically easier than checking every pair?
- What does "overlap" mean for the *current* interval vs. the *last merged* interval?
- How do you handle the empty-input case without a special `length === 0` guard?

Give it a try before scrolling further.

---

## Solution

### Approach

Sort the intervals by start time. Then walk through them once, keeping a `merged` list. For each interval, if it overlaps the last merged interval (its start is `<=` the last one's end), extend the last one's end to the maximum of the two ends; otherwise append it as a new interval.

1. Copy and sort the intervals by start.
2. For each interval, look at the last interval in `merged`.
3. If the last interval is absent or doesn't overlap, push the current one.
4. Otherwise, extend the last interval's end to `max(last.end, current.end)`.

**Time:** O(n log n) for the sort, then O(n). **Space:** O(n) for the result (excluding the sorted copy, which reuses the input in Rust).

### TypeScript

```typescript
function merge(intervals: number[][]): number[][] {
  const sorted = [...intervals].sort((a, b) => a[0]! - b[0]!);
  const merged: number[][] = [];

  for (const interval of sorted) {
    const last = merged[merged.length - 1];
    if (last === undefined || interval[0]! > last[1]!) {
      merged.push(interval);
    } else {
      last[1] = Math.max(last[1]!, interval[1]!);
    }
  }

  return merged;
}

// Quick checks
console.log(JSON.stringify(merge([[1, 3], [2, 6], [8, 10], [15, 18]]))); // [[1,6],[8,10],[15,18]]
console.log(JSON.stringify(merge([[1, 4], [4, 5]])));                    // [[1,5]]
console.log(JSON.stringify(merge([[1, 4], [0, 4]])));                    // [[0,4]]
console.log(JSON.stringify(merge([[1, 4], [5, 6]])));                    // [[1,4],[5,6]]
console.log(JSON.stringify(merge([])));                                  // []
```

### Rust

```rust
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by_key(|iv| iv[0]);
    let mut merged: Vec<Vec<i32>> = Vec::new();

    for interval in intervals {
        match merged.last_mut() {
            Some(last) if interval[0] <= last[1] => {
                last[1] = last[1].max(interval[1]);
            }
            _ => merged.push(interval),
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
        assert_eq!(merge(vec![vec![1, 4], vec![0, 4]]), vec![vec![0, 4]]);
        assert_eq!(merge(vec![vec![1, 4], vec![5, 6]]), vec![vec![1, 4], vec![5, 6]]);
        assert_eq!(merge(vec![]), Vec::<Vec<i32>>::new());
    }
}
```

### Notes on the two implementations

- TypeScript copies the input with `[...intervals]` before sorting so the caller's array is not mutated; Rust instead takes ownership of the `Vec` and sorts it in place, which is free.
- The Rust `match merged.last_mut()` with a `Some(last) if ...` guard combines "is there a last interval" and "does it overlap" into one pattern; the `_` arm covers both "no last interval" and "no overlap".
- `last_mut()` returns `Option<&mut Vec<i32>>`, so `last` is a mutable reference — mutating `last[1]` edits the interval already stored inside `merged`.

## Likely interviewer follow-ups

1. **In-place:** could you merge within the original array instead of allocating a new list? What's the trade-off?
2. **Count vs. merge:** how would you return only the *number* of merged intervals, and does that change the space complexity?
3. **Insert one interval:** given an already-merged sorted list and one new interval, how would you insert and re-merge efficiently?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Sort intervals by their start value.
2. Keep a `merged` list.
3. For each interval, compare it to the last one in `merged`.
4. If it starts after the last one ends, append it. Otherwise, stretch the last one's end to the larger end.

## TypeScript, piece by piece

```typescript
function merge(intervals: number[][]): number[][] {
```
`number[][]` reads "array of arrays of numbers": each interval is itself a `number[]`. The return type is the same shape.

```typescript
const sorted = [...intervals].sort((a, b) => a[0]! - b[0]!);
```
`[...intervals]` makes a shallow copy so we don't mutate the caller's array. `.sort(...)` takes a comparator: returning `a[0] - b[0]` sorts ascending by start. The `!` assertions say "each interval has a start at index `0`", which the constraints guarantee.

```typescript
const last = merged[merged.length - 1];
```
With `noUncheckedIndexedAccess`, reading an out-of-bounds index is typed `T | undefined`. Here that's exactly right: on an empty `merged`, `last` is `undefined`, which we handle next.

```typescript
if (last === undefined || interval[0]! > last[1]!) {
  merged.push(interval);
}
```
The condition is true when there is no previous interval, or when the current interval starts strictly after the last one ends (no overlap). In both cases we start a new merged interval by pushing it.

```typescript
last[1] = Math.max(last[1]!, interval[1]!);
```
Otherwise the intervals overlap, so we extend `last`'s end to whichever is farther right. This mutates the array that's already inside `merged` because `last` is a reference to it.

## Rust, piece by piece

```rust
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
```
`Vec<Vec<i32>>` is a vector of vectors of integers. The parameter is owned (no `&`), so the caller hands over the data and we can sort it in place without copying.

```rust
let mut intervals = intervals;
```
This **shadows** the parameter with a mutable binding of the same name. Rust parameters are immutable by default, and `sort_by_key` needs a mutable vector, so we rebind it as `mut`. The data is moved, not copied.

```rust
intervals.sort_by_key(|iv| iv[0]);
```
`sort_by_key` takes a closure `|iv| iv[0]` that extracts the sort key — here the start of each interval. The vector is sorted ascending by that key. `iv[0]` indexes into a length-2 vector, which the constraints guarantee is safe.

```rust
for interval in intervals {
```
This consumes `intervals` by moving each `Vec<i32>` out one at a time. We no longer need the original vector after this loop.

```rust
match merged.last_mut() {
```
`last_mut()` returns `Option<&mut Vec<i32>>`: `Some` with a mutable reference to the last interval, or `None` if `merged` is empty. Using `last_mut` (not `last`) is what lets us edit the stored interval in the next arm.

```rust
Some(last) if interval[0] <= last[1] => {
    last[1] = last[1].max(interval[1]);
}
```
The `if ...` is a **match guard** that only matches `Some(last)` when the intervals overlap. `last[1].max(interval[1])` returns the larger end, and assigning to `last[1]` edits the interval inside `merged` through the mutable reference.

```rust
_ => merged.push(interval),
```
The catch-all `_` handles both `None` (empty list) and `Some` with no overlap. `interval` is moved into `merged` here, transferring ownership of that `Vec<i32>`.

## Trace: `[[1, 3], [2, 6], [8, 10], [15, 18]]` (should be `[[1, 6], [8, 10], [15, 18]]`)

| Step | interval | last | Action | merged after |
|---|---|---|---|---|
| 1 | `[1, 3]` | none | push | `[[1, 3]]` |
| 2 | `[2, 6]` | `[1, 3]` | `2 <= 3`, extend end to `6` | `[[1, 6]]` |
| 3 | `[8, 10]` | `[1, 6]` | `8 > 6`, push | `[[1, 6], [8, 10]]` |
| 4 | `[15, 18]` | `[8, 10]` | `15 > 10`, push | `[[1, 6], [8, 10], [15, 18]]` |

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Sort by start | `.sort((a, b) => a[0]! - b[0]!)` | `.sort_by_key(\|iv\| iv[0])` |
| Last element | `merged[merged.length - 1]` (may be `undefined`) | `merged.last_mut()` (`Option<&mut _>`) |
| No-last check | `last === undefined` | `None` arm |
| Overlap test | `interval[0]! > last[1]!` | `interval[0] <= last[1]` guard |
| Extend end | `last[1] = Math.max(...)` | `last[1] = last[1].max(...)` |
| Input mutation | copy with `[...intervals]` | take ownership, sort in place |
