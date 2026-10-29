# Product of Array Except Self (deepseek generated)

## Problem

Given an integer array `nums`, return an array `answer` such that `answer[i]` equals the product of all elements of `nums` **except** `nums[i]`. You must solve it **without using division**, and the product is guaranteed to fit in a 32-bit integer.

## Examples

| Input | Output | Why |
|---|---|---|
| `[1,2,3,4]` | `[24,12,8,6]` | e.g. `24 = 2*3*4`, `12 = 1*3*4`, ... |
| `[-1,1,0,-3,3]` | `[0,0,9,0,0]` | Only the element at index 2 has a nonzero product (`-1*1*-3*3 = 9`) |
| `[0,0]` | `[0,0]` | Every other element is zero |
| `[2,3]` | `[3,2]` | Swap the two |

## Constraints

- `2 <= nums.length <= 10^5`
- `-30 <= nums[i] <= 30`
- The product of any prefix or suffix of `nums` fits in a 32-bit integer.

## Things to think about before coding

- If division were allowed, `answer[i]` would just be `total / nums[i]` — why does that break when there are zeros?
- What is `answer[i]` in terms of the elements strictly to the **left** and strictly to the **right** of `i`?
- Can you build all the left-products in one pass and all the right-products in a second pass, reusing a single array?
- Aim for O(n) time and O(1) *extra* space (the output array doesn't count).

Give it a try before scrolling further.

---

## Solution

### Approach

For each index `i`, the answer is `(product of everything left of i) × (product of everything right of i)`. First fill the output with running **prefix** products (left of each index). Then walk backward, multiplying each cell by a running **suffix** product (right of each index). One array holds both partial results.

1. Create `result` of length `n`, filled with `1`s.
2. Left pass: for each `i`, set `result[i] = prefix` (the product so far), then update `prefix *= nums[i]`.
3. Right pass: walk `i` from the end, do `result[i] *= suffix`, then update `suffix *= nums[i]`.
4. Return `result`.

**Time:** O(n), two passes. **Space:** O(1) extra (besides the returned array).

### TypeScript

```typescript
function productExceptSelf(nums: number[]): number[] {
  const n = nums.length;
  const result: number[] = new Array<number>(n).fill(1);

  let prefix = 1;
  for (let i = 0; i < n; i++) {
    result[i] = prefix;
    prefix *= nums[i]!;
  }

  let suffix = 1;
  for (let i = n - 1; i >= 0; i--) {
    result[i] = result[i]! * suffix;
    suffix *= nums[i]!;
  }

  return result;
}

// Quick checks
console.log(productExceptSelf([1, 2, 3, 4]));      // [24, 12, 8, 6]
console.log(productExceptSelf([-1, 1, 0, -3, 3])); // [0, 0, 9, 0, 0]
console.log(productExceptSelf([0, 0]));            // [0, 0]
console.log(productExceptSelf([2, 3]));            // [3, 2]
```

### Rust

```rust
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1i32; n];

    let mut prefix = 1;
    for i in 0..n {
        result[i] = prefix;
        prefix *= nums[i];
    }

    let mut suffix = 1;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= nums[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
        assert_eq!(product_except_self(vec![0, 0]), vec![0, 0]);
    }
}
```

### Notes on the two implementations

- Both avoid a second array by reusing `result`: the left pass writes prefix products, and the right pass multiplies suffix products into the same cells.
- TypeScript needs `nums[i]!` because `noUncheckedIndexedAccess` types indexed reads as possibly `undefined`; in Rust, `nums[i]` is a plain `i32` (it would panic only on an out-of-range index, which the loop prevents).
- In JavaScript, a product like `-1 * 0` can print as `-0`; it is still equal to `0` (`-0 === 0` is `true`), so the zero-heavy example above may display `-0` instead of `0`.

## Likely interviewer follow-ups

1. **Division version:** how would you use division, and how do you handle one zero vs more than one zero?
2. **Space:** why is the output array not counted against the O(1) extra-space requirement?
3. **Generalization:** how would you compute the product except self over a 2D matrix?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. `answer[i]` = (product of everything left of `i`) × (product of everything right of `i`).
2. Walk left to right, storing the running left-product into each cell.
3. Walk right to left, multiplying each cell by the running right-product.
4. Each cell now holds exactly the product of all elements except itself.

## TypeScript, piece by piece

```typescript
const result: number[] = new Array<number>(n).fill(1);
```
`new Array<number>(n)` makes an array of length `n`, and `.fill(1)` sets every cell to `1`. Starting at `1` (the multiplicative identity) is essential: it makes the later multiplications correct for the first and last cells, which have an "empty" product on one side.

```typescript
let prefix = 1;
for (let i = 0; i < n; i++) {
  result[i] = prefix;
  prefix *= nums[i]!;
}
```
`prefix` is the product of all elements seen so far (strictly left of the current index). We store it *before* multiplying in `nums[i]`, so `result[i]` excludes `nums[i]`. `nums[i]!` is a **non-null assertion**: it tells TypeScript the indexed read is safe (the index is always in range).

```typescript
let suffix = 1;
for (let i = n - 1; i >= 0; i--) {
  result[i] = result[i]! * suffix;
  suffix *= nums[i]!;
}
```
Mirror image, going backward. `result[i]` currently holds the left-product; multiplying by `suffix` (the right-product) completes it. `result[i]!` is needed on the *read* because the indexed value is typed `number | undefined`, even though we just wrote it.

```typescript
return result;
```
No extra array was ever allocated, so this satisfies the O(1) extra-space requirement: `result` is the answer itself.

## Rust, piece by piece

```rust
let mut result = vec![1i32; n];
```
`vec![1i32; n]` creates a `Vec<i32>` of length `n`, every element equal to `1`. The `i32` suffix on `1` is a type annotation on the literal so the `Vec`'s element type is unambiguous.

```rust
let mut prefix = 1;
for i in 0..n {
    result[i] = prefix;
    prefix *= nums[i];
}
```
`0..n` is the half-open range of indices. `result[i] = prefix` stores the running left-product; `prefix *= nums[i]` then folds in the current element for the next iteration. In Rust, `nums[i]` is just an `i32` — indexing is bounds-checked and panics only if `i` were out of range.

```rust
for i in (0..n).rev() {
    result[i] *= suffix;
    suffix *= nums[i];
}
```
`(0..n).rev()` iterates the range in reverse (Rust has no `i--`). `result[i] *= suffix` is compound assignment that reads and writes the same cell, and `suffix *= nums[i]` advances the running right-product.

```rust
result
```
The final expression has no `return` and no semicolon. In Rust, the last expression of a function is its return value — here the completed `Vec<i32>`.

## Trace: `[1, 2, 3, 4]` (should be `[24, 12, 8, 6]`)

| Step | Pass | i | result after |
|---|---|---|---|
| 1 | prefix | 0 | `[1, 1, 1, 1]` |
| 2 | prefix | 1 | `[1, 1, 1, 1]` |
| 3 | prefix | 2 | `[1, 1, 2, 1]` |
| 4 | prefix | 3 | `[1, 1, 2, 6]` |
| 5 | suffix | 3 | `[1, 1, 2, 6]` |
| 6 | suffix | 2 | `[1, 1, 8, 6]` |
| 7 | suffix | 1 | `[1, 12, 8, 6]` |
| 8 | suffix | 0 | `[24, 12, 8, 6]` |

After the prefix pass, each cell holds the product of everything to its left. After the suffix pass, each cell also includes everything to its right.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Array of n ones | `new Array(n).fill(1)` | `vec![1i32; n]` |
| Forward loop | `for (let i = 0; i < n; i++)` | `for i in 0..n` |
| Backward loop | `for (let i = n-1; i >= 0; i--)` | `for i in (0..n).rev()` |
| Indexed read type | `number \| undefined` (use `!`) | `i32` (bounds-checked) |
| Compound multiply-assign | `x *= y` | `x *= y` |
| Final return | `return result;` | `result` (implicit) |
| Negative zero | can print `-0` (equals `0`) | no concept (integer) |
