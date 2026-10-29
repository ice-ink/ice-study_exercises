# Remove Duplicates from Sorted Array (deepseek generated)

## Problem

Given a sorted integer array `nums`, remove duplicates in place so that the first `k` elements hold the unique values (each appears once), and return `k`. The elements beyond index `k` do not matter.

## Examples

| Input | Output | Why |
|---|---|---|
| `[1, 1, 2]` | `2` (first two: `[1, 2]`) | Single duplicate of 1 |
| `[0, 0, 1, 1, 1, 2, 2, 3, 3, 4]` | `5` (first five: `[0, 1, 2, 3, 4]`) | Five distinct values |
| `[1]` | `1` | Already unique |
| `[]` | `0` | Empty array |
| `[1, 1, 1, 1]` | `1` | All the same |

## Constraints

- `0 <= nums.length <= 3 * 10^4`
- `-100 <= nums[i] <= 100`
- `nums` is sorted in non-decreasing order.

## Things to think about before coding

- The array is sorted, so duplicates are always adjacent. How does that help?
- How can two pointers (one writing, one reading) place unique values at the front?
- Which pointer gives you the new length at the end?

Give it a try before scrolling further.

---

## Solution

### Approach

Use a slow pointer marking the last written unique value and a fast pointer scanning the array. When the fast pointer finds a value different from the slow pointer's value, advance slow and copy the new value there.

1. If the array is empty, return `0`.
2. Set `slow = 0` (the first element is always unique).
3. For `fast` from 1 to the end: if `nums[fast] != nums[slow]`, do `slow++` and write `nums[slow] = nums[fast]`.
4. Return `slow + 1`.

**Time:** O(n). **Space:** O(1).

### TypeScript

```typescript
function removeDuplicates(nums: number[]): number {
  if (nums.length === 0) return 0;

  let slow = 0;
  for (let fast = 1; fast < nums.length; fast++) {
    if (nums[fast] !== nums[slow]) {
      slow++;
      nums[slow] = nums[fast]!;
    }
  }

  return slow + 1;
}

// Quick checks
console.log(removeDuplicates([1, 1, 2])); // 2
console.log(removeDuplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4])); // 5
console.log(removeDuplicates([1])); // 1
console.log(removeDuplicates([]));  // 0
```

### Rust

```rust
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut slow = 0;
    for fast in 1..nums.len() {
        if nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }

    (slow + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(&nums[..2], &[1, 2][..]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(&nums[..5], &[0, 1, 2, 3, 4][..]);

        let mut nums = vec![1];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(&nums[..1], &[1][..]);

        let mut nums: Vec<i32> = vec![];
        assert_eq!(remove_duplicates(&mut nums), 0);
    }
}
```

### Notes on the two implementations

- The Rust signature takes `&mut Vec<i32>` (a mutable borrow) so it can change the caller's array in place; TypeScript's array argument is already a reference and is mutated directly.
- In Rust, `nums[slow] = nums[fast]` needs no assertion because indexing returns a value; in TypeScript the project's `noUncheckedIndexedAccess` requires a `!` on `nums[fast]`.
- `slow` is the index of the last written unique value, so the count is `slow + 1`.

## Likely interviewer follow-ups

1. **At most two copies:** how would you allow each value to appear *twice* instead of once?
2. **Return the array:** what if you had to return the unique values themselves instead of the length?
3. **Unsorted input:** why does this two-pointer trick depend on the array being sorted?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. `slow` marks the last position holding a confirmed-unique value.
2. `fast` scans ahead. Because the array is sorted, a "new" value appears exactly when `nums[fast]` differs from `nums[slow]`.
3. On a new value, move `slow` forward one and copy the new value there.
4. At the end, unique values occupy indices `0..slow`, so the count is `slow + 1`.

## TypeScript, piece by piece

```typescript
if (nums.length === 0) return 0;
```
Handle the empty array up front so the rest of the code can assume at least one element.

```typescript
let slow = 0;
```
`slow` is the write pointer and also the index of the last unique value so far. The first element is always unique.

```typescript
for (let fast = 1; fast < nums.length; fast++) {
```
`fast` is the read pointer, starting at index 1 (the second element) since index 0 is already "kept."

```typescript
if (nums[fast] !== nums[slow]) {
```
A difference means `fast` found a value we haven't seen yet. Equality means a duplicate, so we skip it. (`nums[fast]` and `nums[slow]` are both `number | undefined` under strict indexing, but the comparison is still valid.)

```typescript
slow++;
nums[slow] = nums[fast]!;
```
Advance the write pointer, then copy the new value into place. `nums[fast]!` asserts the value exists (safe: `fast < nums.length`).

```typescript
return slow + 1;
```
`slow` is the last kept index, so the number of kept values is `slow + 1`.

## Rust, piece by piece

```rust
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
```
`&mut Vec<i32>` is a *mutable borrow*: the function can modify the caller's vector, and the caller keeps ownership. This is how Rust expresses in-place mutation.

```rust
if nums.is_empty() {
    return 0;
}
```
`is_empty()` is the idiomatic way to check for an empty vector. Returning early avoids `slow + 1` on an empty array giving a wrong answer.

```rust
let mut slow = 0;
for fast in 1..nums.len() {
```
`slow` is mutable (it changes). `1..nums.len()` iterates `fast` over the indices 1 through `len - 1`; `nums.len()` is a `usize`, so `fast` is too.

```rust
if nums[fast] != nums[slow] {
    slow += 1;
    nums[slow] = nums[fast];
}
```
Rust indexing returns a plain `i32` (it panics on out-of-bounds but never returns `Option`), so no `unwrap` or assertion. `nums[slow] = nums[fast]` copies the value.

```rust
(slow + 1) as i32
```
`slow` is a `usize`, but the return type is `i32`, so we cast with `as i32`. This expression (no semicolon) is the return value.

## Trace: `[0, 0, 1, 1, 1, 2]` (should be `3`)

| Step | `fast` | `nums[fast]` | `nums[slow]` | Action | Array after |
|---|---|---|---|---|---|
| 1 | 1 | 0 | 0 | equal, skip | `[0, 0, 1, 1, 1, 2]` |
| 2 | 2 | 1 | 0 | differ, slow=1, write 1 | `[0, 1, 1, 1, 1, 2]` |
| 3 | 3 | 1 | 1 | equal, skip | `[0, 1, 1, 1, 1, 2]` |
| 4 | 4 | 1 | 1 | equal, skip | `[0, 1, 1, 1, 1, 2]` |
| 5 | 5 | 2 | 1 | differ, slow=2, write 2 | `[0, 1, 2, 1, 1, 2]` |

Return `slow + 1` = `3`; the first three elements are `[0, 1, 2]`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| In-place mutation | parameter is a reference | `&mut Vec<i32>` |
| Empty check | `nums.length === 0` | `nums.is_empty()` |
| Read an element | `nums[i]!` (non-null) | `nums[i]` (plain value) |
| Loop index | `for (let fast = 1; ...)` | `for fast in 1..nums.len()` |
| Convert index to int | n/a (already `number`) | `(slow + 1) as i32` |
| Write pointer advance | `slow++` | `slow += 1` |
