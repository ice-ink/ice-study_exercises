# Container With Most Water (deepseek generated)

## Problem

You are given an integer array `height` where `height[i]` is the height of a vertical line at position `i`. Choose two lines to form a container with the x-axis such that the container holds the most water.

The amount of water is the area between the two lines: the distance between them (width) multiplied by the shorter line's height (the water can't rise above the shorter wall). Return the maximum area.

## Examples

| Input | Output | Why |
|---|---|---|
| `[1, 8, 6, 2, 5, 4, 8, 3, 7]` | `49` | Bars at index 1 and 8: width 7 × height 7 |
| `[1, 1]` | `1` | Only one container: width 1 × height 1 |
| `[4, 3, 2, 1, 4]` | `16` | Outer bars: width 4 × height 4 |
| `[1, 2, 1]` | `2` | Bars 0 and 2: width 2 × height 1 |

## Constraints

- `2 <= height.length <= 10^5`
- `0 <= height[i] <= 10^4`

## Things to think about before coding

- What determines the height of a container — the taller bar or the shorter one?
- If you start with the widest container (the two ends), which pointer should move inward to have any hope of a larger area?
- Why does moving the shorter bar's pointer never skip a better answer?

Give it a try before scrolling further.

---

## Solution

### Approach

Use two pointers starting at both ends (the widest container). At each step compute the area, update the best answer, then move the pointer at the **shorter** bar inward — a taller container can only come from raising the shorter wall, since width only decreases.

1. Set `left = 0`, `right = n - 1`.
2. While `left < right`, compute `width = right - left` and `water = width * min(height[left], height[right])`.
3. Update the maximum area.
4. Move the shorter of the two bars inward.

**Time:** O(n), each element is considered at most once. **Space:** O(1).

### TypeScript

```typescript
function maxArea(height: number[]): number {
  let left = 0;
  let right = height.length - 1;
  let maxWater = 0;

  while (left < right) {
    const width = right - left;
    const water = width * Math.min(height[left]!, height[right]!);
    if (water > maxWater) maxWater = water;

    if (height[left]! < height[right]!) {
      left++;
    } else {
      right--;
    }
  }

  return maxWater;
}

// Quick checks
console.log(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
console.log(maxArea([1, 1]));                      // 1
console.log(maxArea([4, 3, 2, 1, 4]));             // 16
console.log(maxArea([1, 2, 1]));                   // 2
```

### Rust

```rust
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_water = 0;

    while left < right {
        let width = (right - left) as i32;
        let water = width * height[left].min(height[right]);
        if water > max_water {
            max_water = water;
        }

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(max_area(vec![1, 2, 1]), 2);
    }
}
```

### Notes on the two implementations

- The TypeScript version computes the shorter wall with `Math.min`; Rust uses the method-call form `height[left].min(height[right])` on `i32`.
- In Rust, `right - left` is `usize`, so it's cast to `i32` before multiplying to keep the arithmetic in `i32` land; `height.len() - 1` is safe because the constraints guarantee at least two bars.
- Both use `else { right-- }` when the heights are equal — moving either pointer works, and this choice keeps the logic simple.

## Likely interviewer follow-ups

1. **Equal heights:** why is it correct to move either pointer when the two bars are equal? What if you moved the "wrong" one?
2. **Largest rectangle variant:** how does this differ from "largest rectangle in a histogram", and which data structure does that one need?
3. **Proof:** can you argue formally that the two-pointer approach never misses the optimal pair?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Put one pointer at each end (the widest possible container).
2. Compute the area from the width and the shorter of the two bars.
3. Keep the best area seen.
4. Move the pointer at the shorter bar inward and repeat until the pointers meet.

## TypeScript, piece by piece

```typescript
function maxArea(height: number[]): number {
```
`height: number[]` is the array of bar heights; `: number` declares that we return the maximum area as a single number.

```typescript
let left = 0;
let right = height.length - 1;
let maxWater = 0;
```
The two pointers start at the extremes, giving the maximum possible width. `maxWater` starts at `0` and is the running best area.

```typescript
const width = right - left;
const water = width * Math.min(height[left]!, height[right]!);
```
The container's height is bounded by the **shorter** bar, so we take `Math.min` of the two heights. The `!` assertions mark the index reads as safe because `left` and `right` always point inside the array.

```typescript
if (water > maxWater) maxWater = water;
```
If this container holds more water than any we've seen, remember it. `maxWater` is `let`, so it's reassigned rather than replaced by a new variable.

```typescript
if (height[left]! < height[right]!) {
  left++;
} else {
  right--;
}
```
We advance the pointer at the **shorter** bar. Moving the taller bar can only shrink the width without improving the height, so it can never beat the current best. Moving the shorter bar is the only move that might find a taller wall.

## Rust, piece by piece

```rust
pub fn max_area(height: Vec<i32>) -> i32 {
```
The function takes ownership of the `Vec<i32>` and returns an `i32`. We don't mutate `height`, so ownership is just a convenience here (no borrow needed).

```rust
let mut left = 0;
let mut right = height.len() - 1;
```
`height.len()` returns a `usize`, so `right` is also a `usize`. Both are `mut` because they change each iteration. The `- 1` is safe because the constraints guarantee at least two bars.

```rust
let width = (right - left) as i32;
```
`right - left` is a `usize` (an unsigned integer). We cast it to `i32` with `as` so it can multiply with the `i32` heights without a type mismatch.

```rust
let water = width * height[left].min(height[right]);
```
`height[left].min(height[right])` calls `i32::min` to get the shorter bar. Because `height[left]` returns an `i32` (a `Copy` type), the method-call syntax works directly on the indexed value.

```rust
if height[left] < height[right] {
    left += 1;
} else {
    right -= 1;
}
```
The same "move the shorter bar" rule, with Rust's `+=`/`-=` for the `usize` pointers. Using `else` (not `else if`) handles the equal-height case by moving `right`, which is fine either way.

## Trace: `[1, 8, 6, 2, 5, 4, 8, 3, 7]` (should be `49`)

| Step | left (height) | right (height) | width | shorter height | area | maxWater |
|---|---|---|---|---|---|---|
| 1 | 0 (1) | 8 (7) | 8 | 1 | 8 | 8 |
| 2 | 1 (8) | 8 (7) | 7 | 7 | 49 | 49 |
| 3 | 1 (8) | 7 (3) | 6 | 3 | 18 | 49 |
| 4 | 1 (8) | 6 (8) | 5 | 8 | 40 | 49 |
| 5 | 1 (8) | 5 (4) | 4 | 4 | 16 | 49 |
| 6 | 1 (8) | 4 (5) | 3 | 5 | 15 | 49 |
| 7 | 1 (8) | 3 (2) | 2 | 2 | 4 | 49 |
| 8 | 1 (8) | 2 (6) | 1 | 6 | 6 | 49 |

After step 2 the pointers cross the rest of the array without beating `49`, so `49` is the answer.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Two pointers | `let left = 0; let right = ...` | `let mut left = 0; let mut right = ...` |
| Shorter bar | `Math.min(a, b)` | `a.min(b)` |
| Width type | `right - left` (number) | `(right - left) as i32` (cast from usize) |
| Update max | `if (water > maxWater) maxWater = water;` | `if water > max_water { max_water = water; }` |
| Move pointer | `left++` / `right--` | `left += 1` / `right -= 1` |
| Return value | `return maxWater;` | `max_water` (implicit) |
