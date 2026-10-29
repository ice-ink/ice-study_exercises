# Single Number (deepseek generated)

## Problem

Given a non-empty array of integers `nums` where **every element appears exactly twice except for one**, return the single element that appears only once. You must do it in linear time and constant extra space.

## Examples

| Input | Output | Why |
|---|---|---|
| `[2, 2, 1]` | `1` | `1` appears once, `2` twice |
| `[4, 1, 2, 1, 2]` | `4` | `4` is the only single element |
| `[1]` | `1` | A single element is its own answer |

## Constraints

- `1 <= nums.length <= 3 * 10^4`
- `-3 * 10^4 <= nums[i] <= 3 * 10^4`
- Each element appears exactly twice, except one element that appears once.

## Things to think about before coding

- If you sort or use a hash map you'll solve it, but can you hit O(n) time *and* O(1) space?
- What does `a XOR a` equal? What about `a XOR 0`?
- Does the order of the array matter for the trick you're thinking of?

Give it a try before scrolling further.

---

## Solution

### Approach

XOR every number together. XOR is **commutative** and **associative**, so pairs cancel out regardless of order, and the lone element is what remains.

1. Start with `result = 0`.
2. For each number `n`, update `result = result ^ n`.
3. Every duplicated value appears twice and cancels (`x ^ x = 0`); the single value stays.
4. Return `result`.

**Time:** O(n), one pass. **Space:** O(1), a single accumulator.

### TypeScript

```typescript
function singleNumber(nums: number[]): number {
  let result = 0;

  for (const n of nums) {
    result ^= n;
  }

  return result;
}

// Quick checks
console.log(singleNumber([2, 2, 1])); // 1
console.log(singleNumber([4, 1, 2, 1, 2])); // 4
console.log(singleNumber([1])); // 1
```

### Rust

```rust
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for n in nums {
        result ^= n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(single_number(vec![1]), 1);
    }
}
```

### Notes on the two implementations

- The `^` / `^=` operator is bitwise XOR in both languages; it is **not** exponentiation (JavaScript uses `**` for that, Rust uses `pow`).
- XOR of a number with itself is `0`, and XOR with `0` is the number itself, which is the whole trick.
- Because XOR is order-independent, the duplicates cancel even when they are scattered through the array.

## Likely interviewer follow-ups

1. **Three copies:** how would you solve it if every element appeared exactly *three* times except one (hint: think bits, not XOR)?
2. **Two singles:** how would you find *both* single numbers if two elements appeared once (hint: use XOR to split the array)?
3. **Why XOR:** why does a hash-map or sorting solution fail the O(1)-space / O(n)-time requirement?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Start a running value at zero.
2. XOR the running value with each number in the array.
3. Every number that appears twice cancels itself out (because `x ^ x = 0`).
4. What's left is the one number that appeared only once — return it.

## TypeScript, piece by piece

```typescript
function singleNumber(nums: number[]): number {
```
The function takes an array of numbers and returns a single number. `nums` is guaranteed non-empty by the constraints, so there is always an answer.

```typescript
let result = 0;
```
Start the accumulator at `0`. This is the identity for XOR: `0 ^ n` is just `n`, so the first value passes through unchanged. `let` (not `const`) because we reassign it every iteration.

```typescript
for (const n of nums) {
  result ^= n;
}
```
`^=` is the compound XOR-assignment operator, shorthand for `result = result ^ n`. JavaScript's `^` works on 32-bit integers, so the bitwise math is well-defined here.

```typescript
return result;
```
After the loop, every paired value has canceled to `0`, and the sole leftover is the single number.

## Rust, piece by piece

```rust
pub fn single_number(nums: Vec<i32>) -> i32 {
```
The function takes ownership of a `Vec<i32>` and returns an `i32`. Taking `Vec<i32>` by value (not `&[i32]`) is fine because we only need to read each element once; ownership lets the `for` loop consume the vector.

```rust
let mut result = 0;
```
`0` is inferred as `i32` from the return type and the loop variable. `mut` is required because `^=` mutates it.

```rust
for n in nums {
    result ^= n;
}
```
`for n in nums` iterates by value, moving each `i32` out of the vector (which is cheap, since `i32` is `Copy`). `^=` is bitwise XOR-assignment, identical in meaning to TypeScript's `^=`.

```rust
result
```
The final expression without a semicolon is the return value — Rust's way of saying "return `result`." Adding a semicolon here would change it into a statement and cause a compile error (the function would return nothing).

## Trace: `[4, 1, 2, 1, 2]` (should be `4`)

| Step | `n` | `result` before | `result ^ n` | `result` after |
|---|---|---|---|---|
| start | — | — | — | `0` |
| 1 | 4 | 0 | `0 ^ 4 = 4` | 4 |
| 2 | 1 | 4 | `4 ^ 1 = 5` | 5 |
| 3 | 2 | 5 | `5 ^ 2 = 7` | 7 |
| 4 | 1 | 7 | `7 ^ 1 = 6` | 6 |
| 5 | 2 | 6 | `6 ^ 2 = 4` | 4 |

The `1`s and `2`s cancel out in pairs, leaving `4`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| XOR operator | `^` | `^` |
| XOR-assign | `^=` | `^=` |
| Exponentiation | `**` | `.pow(n)` (not `^`) |
| Loop values | `for (const n of nums)` | `for n in nums` |
| Mutable accumulator | `let result = 0` | `let mut result = 0` |
| Return a value | `return result;` | `result` (no semicolon) |
| Number type | `number` (IEEE double) | `i32` (32-bit int) |
