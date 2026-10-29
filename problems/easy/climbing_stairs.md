# Climbing Stairs (deepseek generated)

## Problem

You are climbing a staircase with `n` steps. Each time you can climb either 1 or 2 steps. Return the number of distinct ways to reach the top.

## Examples

| Input | Output | Why |
|---|---|---|
| `1` | `1` | One step, one way |
| `2` | `2` | 1+1 or 2 |
| `3` | `3` | 1+1+1, 1+2, 2+1 |
| `4` | `5` | Five distinct sequences |
| `5` | `8` | Eight distinct sequences |

## Constraints

- `1 <= n <= 45`

## Things to think about before coding

- What smaller subproblems does "climb `n` steps" depend on?
- Why is `ways(n) = ways(n-1) + ways(n-2)`? Relate the last step to the previous ones.
- Can you compute this with just two numbers instead of an array?

Give it a try before scrolling further.

---

## Solution

### Approach

The number of ways to reach step `n` equals the ways to reach `n-1` (then take a 1-step) plus the ways to reach `n-2` (then take a 2-step). That is the Fibonacci recurrence. Use two variables instead of a full array.

1. Base cases: `n <= 2` returns `n` (1 way for 1 step, 2 ways for 2 steps).
2. Keep `first = 1` (ways(1)) and `second = 2` (ways(2)).
3. For `i` from 3 to `n`, compute `third = first + second`, then shift `first = second`, `second = third`.
4. Return `second`.

**Time:** O(n). **Space:** O(1).

### TypeScript

```typescript
function climbStairs(n: number): number {
  if (n <= 2) return n;

  let first = 1;  // ways to climb 1 step
  let second = 2; // ways to climb 2 steps

  for (let i = 3; i <= n; i++) {
    const third = first + second;
    first = second;
    second = third;
  }

  return second;
}

// Quick checks
console.log(climbStairs(2)); // 2
console.log(climbStairs(3)); // 3
console.log(climbStairs(4)); // 5
console.log(climbStairs(5)); // 8
console.log(climbStairs(1)); // 1
```

### Rust

```rust
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut first = 1;  // ways to climb 1 step
    let mut second = 2; // ways to climb 2 steps

    for _ in 3..=n {
        let third = first + second;
        first = second;
        second = third;
    }

    second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}
```

### Notes on the two implementations

- The recurrence is exactly the Fibonacci sequence shifted by one; `ways(1) = 1` and `ways(2) = 2` are the base cases.
- Both solutions roll two variables forward instead of allocating an array, keeping space O(1).
- Rust's `for _ in 3..=n` uses `_` because the loop counter is unused; the `..=` range is inclusive of `n`.

## Likely interviewer follow-ups

1. **Steps of 1, 2, or 3:** how does the recurrence change if you can also take 3 steps?
2. **Faster than O(n):** how could matrix exponentiation (or a closed form) compute this in O(log n)?
3. **Cost array:** what changes if each step has a cost and you must minimize total cost?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. `ways(1) = 1`, `ways(2) = 2`.
2. For `n >= 3`, `ways(n) = ways(n-1) + ways(n-2)`: the last step is either a 1-step from `n-1` or a 2-step from `n-2`.
3. Roll two variables (`first`, `second`) forward until you reach `n`.
4. Return the value for `n`.

## TypeScript, piece by piece

```typescript
if (n <= 2) return n;
```
The base cases: `n = 1` has 1 way, `n = 2` has 2 ways. Returning early avoids the loop for tiny inputs.

```typescript
let first = 1;  // ways to climb 1 step
let second = 2; // ways to climb 2 steps
```
`first` and `second` hold the two most recent values of the sequence. `let` because both are reassigned each iteration.

```typescript
for (let i = 3; i <= n; i++) {
```
Start at 3 because 1 and 2 are already known. `i <= n` includes `n`.

```typescript
const third = first + second;
first = second;
second = third;
```
`third` is `ways(i)`. Then shift: the old `second` becomes `first`, and the new value becomes `second`, ready for the next iteration.

```typescript
return second;
```
After the loop, `second` holds `ways(n)`.

## Rust, piece by piece

```rust
if n <= 2 {
    return n;
}
```
Early return for the base cases. In Rust you need an explicit `return` here because the function would otherwise fall through to the implicit-return tail.

```rust
let mut first = 1;
let mut second = 2;
```
Both must be `mut` since they're reassigned. Rust's default immutability would reject `first = second` otherwise.

```rust
for _ in 3..=n {
```
`3..=n` is an inclusive range (it includes `n`). The `_` is the conventional name for an unused loop variable; we need only the count of iterations, not the value of `i`.

```rust
let third = first + second;
first = second;
second = third;
```
`third` is immutable (`let`) because it's only read once. The `first = second` and `second = third` lines require the `mut` from earlier.

```rust
second
```
The last expression is returned implicitly (no semicolon, no `return`).

## Trace: `n = 4` (should be `5`)

| Iteration | `i` | `first` | `second` |
|---|---|---|---|
| (start) | — | 1 | 2 |
| 1 | 3 | 2 | 3 |
| 2 | 4 | 3 | 5 |

After the loop, `second` is `5`, which is `ways(4)`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Early return | `if (n <= 2) return n;` | `if n <= 2 { return n; }` |
| Inclusive range | `i <= n` in a `for` | `3..=n` |
| Unused loop var | use `i` and ignore it | `_` |
| Shift two values | `first = second; second = third;` | same |
| Return a value | `return second` | `second` (implicit) |
