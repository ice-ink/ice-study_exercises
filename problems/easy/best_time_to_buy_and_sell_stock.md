# Best Time to Buy and Sell Stock (deepseek generated)

## Problem

Given an array `prices` where `prices[i]` is the price of a stock on day `i`, choose one day to buy and one later day to sell. Return the maximum profit you can achieve. If no profit is possible, return `0`.

## Examples

| Input | Output | Why |
|---|---|---|
| `[7, 1, 5, 3, 6, 4]` | `5` | Buy at 1, sell at 6 |
| `[7, 6, 4, 3, 1]` | `0` | Prices only fall; no profit possible |
| `[1, 2, 3, 4, 5]` | `4` | Buy at 1, sell at 5 |
| `[2, 4, 1]` | `2` | Best is buy 2, sell 4 — not the global min |
| `[3, 3, 3]` | `0` | Flat prices, no gain |
| `[1]` | `0` | Can't buy and sell on the same day |

## Constraints

- `1 <= prices.length <= 10^5`
- `0 <= prices[i] <= 10^4`

## Things to think about before coding

- What single value do you need to remember as you scan from left to right?
- Why does "buy at the global minimum, sell at the global maximum" fail? (Try `[2, 4, 1]`.)
- Can you get the answer in one pass with constant extra memory?

Give it a try before scrolling further.

---

## Solution

### Approach

Scan left to right while tracking the lowest price seen so far and the best profit seen so far. For each day, the best possible sale "today" is `price - lowestSoFar`; keep the maximum of that.

1. Initialize `minPrice = Infinity` (or `i32::MAX`) and `best = 0`.
2. For each price, update `minPrice` to the smaller of itself and `price`.
3. Update `best` to the larger of itself and `price - minPrice`.
4. Return `best`.

**Time:** O(n), one pass. **Space:** O(1).

### TypeScript

```typescript
function maxProfit(prices: number[]): number {
  let minPrice = Infinity;
  let best = 0;

  for (const price of prices) {
    minPrice = Math.min(minPrice, price);
    best = Math.max(best, price - minPrice);
  }

  return best;
}

// Quick checks
console.log(maxProfit([7, 1, 5, 3, 6, 4])); // 5
console.log(maxProfit([7, 6, 4, 3, 1]));    // 0
console.log(maxProfit([1, 2]));             // 1
console.log(maxProfit([1]));                // 0
```

### Rust

```rust
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut best = 0;

    for price in prices {
        min_price = min_price.min(price);
        best = best.max(price - min_price);
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(vec![1, 2]), 1);
        assert_eq!(max_profit(vec![2, 4, 1]), 2);
        assert_eq!(max_profit(vec![1]), 0);
    }
}
```

### Notes on the two implementations

- TypeScript starts `minPrice` at `Infinity` so the first price always becomes the new minimum; Rust uses `i32::MAX` for the same job.
- `Math.min`/`Math.max` are free functions in TypeScript, but in Rust `.min()`/`.max()` are methods on integers (from the `Ord` trait).
- The Rust `for price in prices` *moves* the vector; that's fine because we never use `prices` again.

## Likely interviewer follow-ups

1. **Multiple transactions:** what changes if you can buy and sell as many times as you want (holding at most one share)?
2. **Return the days:** how would you also return the buy and sell indices?
3. **Two-pointer variant:** could a buy/sell pointer approach work, and what is its trade-off?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Keep `minPrice`, the cheapest price seen so far.
2. Keep `best`, the best profit seen so far.
3. For each price, first lower `minPrice` if needed, then compute today's profit as `price - minPrice` and raise `best` if needed.
4. Return `best`.

## TypeScript, piece by piece

```typescript
function maxProfit(prices: number[]): number {
```
`prices: number[]` means "an array of numbers." The `: number` after the closing parenthesis is the return type.

```typescript
let minPrice = Infinity;
```
`Infinity` is a number bigger than every real number, so on the first loop iteration every real price is smaller and becomes the new minimum. `let` is used because this value changes.

```typescript
for (const price of prices) {
```
`for...of` walks the array giving each *value* in turn (`const` because `price` is never reassigned). Don't confuse it with `for...in`, which gives indices.

```typescript
minPrice = Math.min(minPrice, price);
```
`Math.min` returns the smaller of its two arguments. After this line, `minPrice` is the cheapest price from day 0 through today.

```typescript
best = Math.max(best, price - minPrice);
```
`price - minPrice` is the profit you'd get if you had bought at the cheapest earlier day and sold today. `Math.max` keeps the largest such profit.

```typescript
return best;
```
If the array only ever falls, `best` stays `0`, which matches the "return 0 if no profit" rule.

## Rust, piece by piece

```rust
pub fn max_profit(prices: Vec<i32>) -> i32 {
```
- `pub` makes the function callable from outside this module.
- `Vec<i32>` is an owned, growable array of 32-bit integers; taking it by value means the function *owns* `prices`.
- `-> i32` is the return type.

```rust
let mut min_price = i32::MAX;
```
`i32::MAX` is the largest possible 32-bit integer (2,147,483,647) — Rust's stand-in for `Infinity` here. `mut` is required because the value is reassigned; Rust variables are immutable by default.

```rust
for price in prices {
```
This consumes (moves) the vector: each `price` is a plain `i32`. Moving is fine because the function never needs `prices` afterward.

```rust
min_price = min_price.min(price);
```
`.min()` is a *method* on integers (from the `Ord` trait) that returns the smaller of `self` and the argument. Notice there's no `Math.` prefix like in TypeScript.

```rust
best = best.max(price - min_price);
```
`.max()` returns the larger of the two. `price - min_price` is today's profit if you bought at the cheapest earlier price.

```rust
best
```
No semicolon and no `return`. In Rust, the last expression of a function body is the return value; adding a `;` would turn it into a statement and cause a compile error.

## Trace: `[7, 1, 5, 3, 6, 4]` (should be `5`)

| Step | `price` | `minPrice` | `best` |
|---|---|---|---|
| 1 | 7 | 7 | 0 |
| 2 | 1 | 1 | 0 |
| 3 | 5 | 1 | 4 |
| 4 | 3 | 1 | 4 |
| 5 | 6 | 1 | 5 |
| 6 | 4 | 1 | 5 |

The best profit `5` comes from buying at `1` and selling at `6`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| "Largest number" sentinel | `Infinity` | `i32::MAX` |
| Minimum of two numbers | `Math.min(a, b)` | `a.min(b)` |
| Maximum of two numbers | `Math.max(a, b)` | `a.max(b)` |
| Loop over array values | `for (const x of arr)` | `for x in arr` (moves) |
| Mutable variable | `let` | `let mut` |
| Return a value | `return best` | `best` (implicit) |
