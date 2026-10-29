# Rotate Array (deepseek generated)

## Problem

Given an integer array `nums`, rotate the array **to the right** by `k` steps, mutating it in place.

Rotating right by `k` steps means the last `k` elements move to the front and every other element shifts right. If `k` is larger than the array length, the rotation wraps around.

## Examples

| Input | Output | Why |
|---|---|---|
| `[1,2,3,4,5,6,7]`, `k = 3` | `[5,6,7,1,2,3,4]` | Last 3 elements move to the front |
| `[-1,-100,3,99]`, `k = 2` | `[3,99,-1,-100]` | Last 2 elements move to the front |
| `[1,2,3]`, `k = 4` | `[3,1,2]` | `k` wraps: `4 % 3 = 1` step |
| `[1,2,3,4]`, `k = 0` | `[1,2,3,4]` | No rotation |

## Constraints

- `1 <= nums.length <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `0 <= k <= 10^5`

## Things to think about before coding

- What happens when `k` is larger than the array length, and how do you reduce it to a meaningful amount?
- Is there a way to rearrange the array with only swaps and no extra array (O(1) extra space)?
- What sequence of reversals turns `[a, b]` into `[b, a]`?

Give it a try before scrolling further.

---

## Solution

### Approach

Normalize `k` with `k % n`, then perform **three reversals**: reverse the whole array, reverse the first `k` elements, and reverse the remaining `n - k` elements. This moves the last `k` elements to the front in place.

1. If the array is empty, return immediately.
2. Reduce `k` to `k % n` so it stays within bounds.
3. Reverse the whole array.
4. Reverse the first `k` elements.
5. Reverse the remaining elements.

**Time:** O(n) — three linear passes. **Space:** O(1) — swaps only.

### TypeScript

```typescript
function rotate(nums: number[], k: number): void {
  const n = nums.length;
  if (n === 0) return;
  const steps = k % n;

  reverse(nums, 0, n - 1);
  reverse(nums, 0, steps - 1);
  reverse(nums, steps, n - 1);
}

function reverse(nums: number[], start: number, end: number): void {
  while (start < end) {
    const tmp = nums[start]!;
    nums[start] = nums[end]!;
    nums[end] = tmp;
    start++;
    end--;
  }
}

// Quick checks
const a1 = [1, 2, 3, 4, 5, 6, 7];
rotate(a1, 3);
console.log(a1); // [5, 6, 7, 1, 2, 3, 4]

const a2 = [-1, -100, 3, 99];
rotate(a2, 2);
console.log(a2); // [3, 99, -1, -100]

const a3 = [1, 2, 3];
rotate(a3, 4);
console.log(a3); // [3, 1, 2]

const a4 = [1, 2, 3, 4];
rotate(a4, 0);
console.log(a4); // [1, 2, 3, 4]
```

### Rust

```rust
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    if n == 0 {
        return;
    }
    let steps = (k % n as i32) as usize;

    nums.reverse();
    nums[..steps].reverse();
    nums[steps..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut a, 3);
        assert_eq!(a, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut b = vec![-1, -100, 3, 99];
        rotate(&mut b, 2);
        assert_eq!(b, vec![3, 99, -1, -100]);

        let mut c = vec![1, 2, 3];
        rotate(&mut c, 4);
        assert_eq!(c, vec![3, 1, 2]);

        let mut d = vec![1, 2, 3, 4];
        rotate(&mut d, 0);
        assert_eq!(d, vec![1, 2, 3, 4]);
    }
}
```

### Notes on the two implementations

- TypeScript needs a hand-written `reverse` helper because `Array.prototype.reverse` always reverses the entire array; Rust gets a slice `.reverse()` for free, so we reverse `nums[..steps]` and `nums[steps..]` directly.
- In Rust, `k % n as i32` relies on `as` binding tighter than `%`, so it's `k % (n as i32)`; the result is cast back to `usize` for slice indexing.
- Both return nothing (`void` / no return) and mutate their input, which is the whole point of the "in place" requirement.

## Likely interviewer follow-ups

1. **Left rotation:** how would you rotate the array to the *left* by `k` instead, and how does the reversal order change?
2. **Cyclic replacement:** what is the alternative O(n)/O(1) approach using element cycles, and when does it beat three reversals?
3. **Overflow:** why is `k % n` necessary for correctness, and what would a very large `k` do to a naive "shift one step k times" solution?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Reduce `k` to `k % n` so it fits inside the array.
2. Reverse the entire array.
3. Reverse just the first `k` elements.
4. Reverse the remaining `n - k` elements.

## TypeScript, piece by piece

```typescript
function rotate(nums: number[], k: number): void {
```
The return type is `void` because the function mutates `nums` in place and returns nothing. `nums` is passed by reference, so mutations are visible to the caller.

```typescript
const n = nums.length;
if (n === 0) return;
const steps = k % n;
```
`n` is the length. We guard the empty case because reversing and indexing would misbehave. `steps = k % n` folds `k` down: rotating a length-3 array by `4` is the same as rotating by `1`.

```typescript
reverse(nums, 0, n - 1);
reverse(nums, 0, steps - 1);
reverse(nums, steps, n - 1);
```
Three reversals do the work. Note `steps - 1` can be `-1` when `steps` is `0`; the helper's `while (start < end)` guard makes `reverse(nums, 0, -1)` a no-op, so `k = 0` still works.

```typescript
function reverse(nums: number[], start: number, end: number): void {
  while (start < end) {
    const tmp = nums[start]!;
    nums[start] = nums[end]!;
    nums[end] = tmp;
    start++;
    end--;
  }
}
```
This swaps the two ends and walks inward. `tmp` holds one value while we overwrite it. The `!` assertions mark the index reads as safe because `start` and `end` stay inside `[0, n - 1]`.

## Rust, piece by piece

```rust
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
```
`&mut Vec<i32>` is a **mutable reference**: the function borrows the caller's vector and may change it, without taking ownership. The caller still owns the data afterward.

```rust
let n = nums.len();
if n == 0 {
    return;
}
```
`len()` returns a `usize` (the unsigned pointer-sized integer Rust uses for sizes and indexes). We guard the empty case so later slicing never indexes into nothing.

```rust
let steps = (k % n as i32) as usize;
```
`n as i32` casts the length to `i32` so it can be used with `k`. Because `as` binds tighter than `%`, this is `k % (n as i32)`. The result is cast to `usize` for slice indexing.

```rust
nums.reverse();
```
`Vec::reverse` reverses the whole vector in place. This is the first of the three reversals.

```rust
nums[..steps].reverse();
nums[steps..].reverse();
```
`nums[..steps]` and `nums[steps..]` are **slices** (views into the vector). Calling `.reverse()` on each reverses just those ranges. `steps` must be a `usize`, which is why we cast earlier. When `steps` is `0`, the first slice is empty and its reversal is a no-op.

## Trace: `[1, 2, 3, 4, 5, 6, 7]`, `k = 3` (should be `[5, 6, 7, 1, 2, 3, 4]`)

| Step | Action | Array after |
|---|---|---|
| 0 | start | `[1, 2, 3, 4, 5, 6, 7]` |
| 1 | reverse whole `[0..6]` | `[7, 6, 5, 4, 3, 2, 1]` |
| 2 | reverse first `3` (`[0..2]`) | `[5, 6, 7, 4, 3, 2, 1]` |
| 3 | reverse remaining `4` (`[3..6]`) | `[5, 6, 7, 1, 2, 3, 4]` |

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Mutate in place | parameter `nums: number[]` | parameter `nums: &mut Vec<i32>` |
| No return value | `: void` | no return type |
| Length | `nums.length` | `nums.len()` |
| Reverse a range | hand-written `reverse` helper | `slice.reverse()` |
| Reduce k | `k % n` | `(k % n as i32) as usize` |
| Swap elements | temp variable | (built into `.reverse()`) |
