# Plus One (deepseek generated)

## Problem

Given a non-negative integer represented as an array of its decimal digits (most significant digit first), increment the integer by one and return the resulting digits as an array.

## Examples

| Input | Output | Why |
|---|---|---|
| `[1, 2, 3]` | `[1, 2, 4]` | 123 + 1 = 124 |
| `[4, 3, 2, 1]` | `[4, 3, 2, 2]` | 4321 + 1 = 4322 |
| `[9]` | `[1, 0]` | Carry creates a new leading digit |
| `[9, 9, 9]` | `[1, 0, 0, 0]` | All 9s → all zeros with a leading 1 |
| `[0]` | `[1]` | 0 + 1 = 1 |

## Constraints

- `1 <= digits.length <= 100`
- `0 <= digits[i] <= 9`
- `digits` has no leading zeros except the number `0` itself.

## Things to think about before coding

- Where does the increment "start"? Which digit changes first?
- What happens to a `9` when you add one, and where does the carry go?
- Do you always need to touch every digit, or can you sometimes stop early?

Give it a try before scrolling further.

---

## Solution

### Approach

Add one from the least significant digit (the end), propagating a carry. If a digit is `< 9`, increment it and return immediately. Otherwise set it to `0` and continue left. If you exit the loop, every digit was `9`, so prepend a `1`.

1. Loop `i` from the last index down to `0`.
2. If `digits[i] < 9`, add 1 to it and return `digits`.
3. Else set `digits[i] = 0`.
4. After the loop (all were 9), return `[1, ...digits]`.

**Time:** O(n). **Space:** O(1), except O(n) in the all-9s case for the new array.

### TypeScript

```typescript
function plusOne(digits: number[]): number[] {
  for (let i = digits.length - 1; i >= 0; i--) {
    const digit = digits[i]!;
    if (digit < 9) {
      digits[i] = digit + 1;
      return digits;
    }
    digits[i] = 0;
  }

  return [1, ...digits];
}

// Quick checks
console.log(plusOne([1, 2, 3])); // [1, 2, 4]
console.log(plusOne([4, 3, 2, 1])); // [4, 3, 2, 2]
console.log(plusOne([9])); // [1, 0]
console.log(plusOne([9, 9, 9])); // [1, 0, 0, 0]
```

### Rust

```rust
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }

    let mut result = vec![1];
    result.extend_from_slice(&digits);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(plus_one(vec![0]), vec![1]);
    }
}
```

### Notes on the two implementations

- Both mutate the input array in place for the no-carry case and return early; only the all-9s case allocates a new array.
- TypeScript's `digits[i]!` is a non-null assertion (the project's `noUncheckedIndexedAccess` types `digits[i]` as `number | undefined`); it is safe because `i` is always a valid index. Rust indexing returns a plain `i32`, so no assertion is needed.
- In Rust, `(0..digits.len()).rev()` iterates from the last index down to `0`; `.rev()` reverses the range.

## Likely interviewer follow-ups

1. **No mutation:** how would you return a new array without modifying the input?
2. **Arbitrary base:** what changes if the digits are base 2 or base 16 instead of base 10?
3. **Subtract one:** what's the analogous algorithm for decrementing by one?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Start at the last digit (the "ones" place).
2. If it's less than 9, just add 1 and you're done.
3. If it's 9, it becomes 0 and the carry moves left one place.
4. If every digit was 9, all become 0 and you prepend a 1.

## TypeScript, piece by piece

```typescript
for (let i = digits.length - 1; i >= 0; i--) {
```
`digits.length - 1` is the last index; `i >= 0` keeps going while valid; `i--` moves one digit left (toward the most significant digit).

```typescript
const digit = digits[i]!;
```
Under `noUncheckedIndexedAccess`, `digits[i]` is typed `number | undefined`, so it can't be used as a number directly. The trailing `!` is a non-null assertion that tells TypeScript "trust me, this index exists." It's safe here because the loop guarantees a valid index.

```typescript
if (digit < 9) {
  digits[i] = digit + 1;
  return digits;
}
```
If the digit is 0–8, adding 1 needs no carry, so write the new value back and return the (mutated) original array.

```typescript
digits[i] = 0;
```
A `9` plus one becomes `10`, so this place becomes `0` and the `1` carries to the next place — handled by the next loop iteration.

```typescript
return [1, ...digits];
```
Only reached when every digit was `9` (they're all `0` now). `[1, ...digits]` builds a new array with `1` in front and the existing zeros spread after it. The spread `...` copies the array's elements.

## Rust, piece by piece

```rust
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
```
`mut` on the parameter means we can modify `digits` in place (write values back). Without `mut`, `digits[i] = 0` would not compile.

```rust
for i in (0..digits.len()).rev() {
```
`0..digits.len()` is the range of indices; `.rev()` makes it run backwards from `len - 1` down to `0`. `i` is a `usize` (Rust's index type), so it can index directly.

```rust
if digits[i] < 9 {
    digits[i] += 1;
    return digits;
}
```
Rust indexing returns a plain `i32` (not an `Option`), so no `.unwrap()` or assertion is needed. `+= 1` adds one in place.

```rust
let mut result = vec![1];
result.extend_from_slice(&digits);
result
```
`vec![1]` makes a one-element vector. `extend_from_slice(&digits)` appends every element of `digits` (all zeros now) onto it, producing `[1, 0, 0, ...]`. The final `result` (no semicolon) is the return value.

## Trace: `[9, 9]` (should be `[1, 0, 0]`)

| Step | `i` | `digits[i]` | Action | Array after |
|---|---|---|---|---|
| 1 | 1 | 9 | set to 0 | `[9, 0]` |
| 2 | 0 | 9 | set to 0 | `[0, 0]` |
| — | loop ends | — | all were 9, prepend 1 | `[1, 0, 0]` |

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Reverse index loop | `for (let i = len - 1; i >= 0; i--)` | `for i in (0..len).rev()` |
| Read an element | `digits[i]!` (non-null) | `digits[i]` (plain value) |
| Prepend a value | `[1, ...digits]` | `vec![1]` + `extend_from_slice` |
| Mutate parameter | mutable by default | need `mut` on the parameter |
| Index type | `number` | `usize` |
