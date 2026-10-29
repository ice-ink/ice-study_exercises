# Coin Change (deepseek generated)

## Problem

You are given an array `coins` of distinct integers (the denominations) and an integer `amount`. Return the **fewest number of coins** needed to make up that amount, or `-1` if the amount cannot be made with the given coins. You may use each denomination an unlimited number of times.

For example, with coins `[1, 2, 5]` and amount `11`, the answer is `3` (`5 + 5 + 1`).

## Examples

| Input | Output | Why |
|---|---|---|
| `coins = [1,2,5]`, `amount = 11` | `3` | `5 + 5 + 1` |
| `coins = [2]`, `amount = 3` | `-1` | `3` can't be built from `2`s |
| `coins = [1]`, `amount = 0` | `0` | Zero coins make zero |
| `coins = [2,5,10,1]`, `amount = 27` | `4` | `10 + 10 + 5 + 2` |
| `coins = [186,419,83,408]`, `amount = 6249` | `20` | Greedy (largest first) fails; DP is required |

## Constraints

- `1 <= coins.length <= 12`
- `1 <= coins[i] <= 2^31 - 1`
- All values in `coins` are unique.
- `0 <= amount <= 10^4`

## Things to think about before coding

- Why does a greedy "take the biggest coin first" approach fail on the last example above?
- If you knew the answer for every amount smaller than `a`, how would you compute the answer for `a`?
- What value should you use to represent "impossible" while you fill the table?
- What is the answer when `amount` is `0`?

Give it a try before scrolling further.

---

## Solution

### Approach

This is classic **unbounded knapsack** solved with bottom-up dynamic programming. Let `dp[a]` be the fewest coins to make amount `a`. The key recurrence: to make `a`, pick any coin `c` as the last coin, and the best answer is `dp[a - c] + 1`; take the minimum over all usable coins.

1. Create `dp` of length `amount + 1`, initialized to `Infinity` (or a large sentinel), and set `dp[0] = 0` (zero coins make zero).
2. For each `a` from `1` to `amount`, for each coin `c <= a`, try `dp[a] = min(dp[a], dp[a - c] + 1)`.
3. If `dp[amount]` is still the sentinel, no combination exists — return `-1`; otherwise return `dp[amount]`.

**Time:** O(amount · coins.length). **Space:** O(amount).

### TypeScript

```typescript
function coinChange(coins: number[], amount: number): number {
  const dp = new Array<number>(amount + 1).fill(Infinity);
  dp[0] = 0;

  for (let a = 1; a <= amount; a++) {
    for (const coin of coins) {
      if (coin <= a) {
        dp[a] = Math.min(dp[a]!, dp[a - coin]! + 1);
      }
    }
  }

  return dp[amount]! === Infinity ? -1 : dp[amount]!;
}

// Quick checks
console.log(coinChange([1, 2, 5], 11)); // 3  (5+5+1)
console.log(coinChange([2], 3)); // -1
console.log(coinChange([1], 0)); // 0
console.log(coinChange([2, 5, 10, 1], 27)); // 4  (10+10+5+2)
console.log(coinChange([186, 419, 83, 408], 6249)); // 20
```

### Rust

```rust
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;

    for a in 1..=amount {
        for &coin in &coins {
            let coin = coin as usize;
            if coin <= a && dp[a - coin] != i32::MAX {
                dp[a] = dp[a].min(dp[a - coin] + 1);
            }
        }
    }

    if dp[amount] == i32::MAX {
        -1
    } else {
        dp[amount]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
        assert_eq!(coin_change(vec![2, 5, 10, 1], 27), 4);
        assert_eq!(coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
```

### Notes on the two implementations

- TypeScript uses `Infinity` as the "unreachable" sentinel; Rust uses `i32::MAX` for the same role (integers have no infinity).
- Rust must convert `amount` and `coin` to `usize` to use them as `Vec` indices; the `coin <= a` check runs first, so a coin larger than the current amount is skipped.
- In Rust, the guard `dp[a - coin] != i32::MAX` also prevents overflow: adding `1` to `i32::MAX` would otherwise wrap around.

## Likely interviewer follow-ups

1. **Which coins:** how would you reconstruct the actual list of coins, not just the count?
2. **Space reduction:** the current DP is O(amount); can you explain why you can't easily drop to O(1) here (unlike Fibonacci)?
3. **Overflow:** if `amount` could be up to `10^9`, what would break and how would you adapt?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Build an array `dp` where `dp[a]` = the fewest coins for amount `a`; start with `dp[0] = 0` and everything else "impossible".
2. For every amount from 1 upward, look at each coin that fits.
3. "If I use this coin as the last one, the rest costs `dp[a - coin]`, so the total is `dp[a - coin] + 1`." Keep the smallest total.
4. At the end, read `dp[amount]`; if it's still "impossible", return `-1`.

## TypeScript, piece by piece

```typescript
const dp = new Array<number>(amount + 1).fill(Infinity);
dp[0] = 0;
```
`new Array<number>(amount + 1)` makes an array with `amount + 1` slots, and `.fill(Infinity)` fills every slot with `Infinity` — our "impossible" marker. Slot `dp[a]` will hold the answer for amount `a`, so we need indices `0..amount`. `dp[0] = 0` is the base case: zero coins make amount zero.

```typescript
for (let a = 1; a <= amount; a++) {
  for (const coin of coins) {
    if (coin <= a) {
      dp[a] = Math.min(dp[a]!, dp[a - coin]! + 1);
    }
  }
}
```
The outer loop builds answers in increasing order, so when we compute `dp[a]`, every smaller `dp[a - coin]` is already finalized. The inner loop tries each coin. `coin <= a` skips coins too big to use. `Math.min` picks the better of the current best and "use this coin" (`dp[a - coin] + 1`). The `!` after `dp[a]` and `dp[a - coin]` is a non-null assertion: under `noUncheckedIndexedAccess` these read as `number | undefined`, but the indices are provably in range.

```typescript
return dp[amount]! === Infinity ? -1 : dp[amount]!;
```
A **ternary** (`condition ? A : B`) means "if true return `A`, else return `B`." If `dp[amount]` never got updated from `Infinity`, no combination exists, so we return `-1`; otherwise we return the count. The `!` again asserts the index is valid.

## Rust, piece by piece

```rust
let amount = amount as usize;
let mut dp = vec![i32::MAX; amount + 1];
dp[0] = 0;
```
`amount as usize` converts the signed `i32` to an unsigned `usize`, which is what array/Vec lengths and indices require. `vec![i32::MAX; amount + 1]` is the "vec macro with repetition": it creates a `Vec` of length `amount + 1` where every element is `i32::MAX` (our impossible sentinel). `dp[0] = 0` is the base case.

```rust
for a in 1..=amount {
    for &coin in &coins {
```
`1..=amount` is an **inclusive range** (the `=` means "include the end"), so `a` runs 1, 2, …, `amount`. `for &coin in &coins` iterates over `&coins` and **dereferences** each `&i32` with the leading `&`, so `coin` is a plain `i32` copy. This is the idiomatic way to iterate a `Vec` by value without taking ownership.

```rust
let coin = coin as usize;
if coin <= a && dp[a - coin] != i32::MAX {
    dp[a] = dp[a].min(dp[a - coin] + 1);
}
```
`coin` is converted to `usize` for indexing. The condition has two parts: `coin <= a` keeps `a - coin` from underflowing (it stays ≥ 0), and `dp[a - coin] != i32::MAX` skips the "impossible" states so we never compute `i32::MAX + 1` (which would overflow). `dp[a].min(...)` returns the smaller value.

```rust
if dp[amount] == i32::MAX {
    -1
} else {
    dp[amount]
}
```
Rust's `if`/`else` is an **expression**, so each branch's value becomes the function's return value. If the amount is still unreachable we return `-1`; otherwise the count. (Note these are block expressions, not `return` statements — the last value in each block is what's produced.)

## Trace: `coins = [1,2,5]`, `amount = 5` (should be `1`)

| Step | `a` | coin tried | `dp[a - coin]` | New `dp[a]` |
|---|---|---|---|---|
| 1 | 1 | 1 | `dp[0]=0` | `1` |
| 2 | 2 | 1 | `dp[1]=1` | `2` |
| 3 | 2 | 2 | `dp[0]=0` | `1` |
| 4 | 3 | 1 | `dp[2]=1` | `2` |
| 5 | 3 | 2 | `dp[1]=1` | `2` |
| 6 | 4 | 2 | `dp[2]=1` | `2` |
| 7 | 5 | 1 | `dp[4]=2` | `3` |
| 8 | 5 | 2 | `dp[3]=2` | `3` |
| 9 | 5 | 5 | `dp[0]=0` | `1` |

`dp[5]` ends at `1` (a single 5-coin), and coins `> 5` are skipped by the `coin <= a` guard.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Impossible sentinel | `Infinity` | `i32::MAX` |
| Array of size `n+1` | `new Array<number>(amount + 1).fill(...)` | `vec![x; amount + 1]` |
| Loop `1..=n` | `for (let a = 1; a <= amount; a++)` | `for a in 1..=amount` |
| Iterate coins by value | `for (const coin of coins)` | `for &coin in &coins` |
| Min of two | `Math.min(x, y)` | `x.min(y)` |
| Index type | `number` (no cast needed) | `usize` (`as usize` cast) |
| Conditional return | `cond ? a : b` | `if cond { a } else { b }` |
