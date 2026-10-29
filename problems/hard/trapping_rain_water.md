# Trapping Rain Water (deepseek generated)

## Problem

Given an array `height` of non-negative integers where each element is the height of a vertical bar of width 1, return how much **water** can be trapped between the bars after it rains.

Water is trapped at a position when there is a taller bar on both sides; the amount above that position is `min(leftTallest, rightTallest) - height[i]`, summed over all positions.

## Examples

| Input | Output | Why |
|---|---|---|
| `[0,1,0,2,1,0,1,3,2,1,2,1]` | `6` | Classic example; 6 unit cells are filled |
| `[4,2,0,3,2,5]` | `9` | Water pools between the `4` and `5` walls |
| `[1,2,3]` | `0` | Monotonically increasing: no wall on the left to trap anything |
| `[3,2,1]` | `0` | Monotonically decreasing: no wall on the right |
| `[]` | `0` | Empty input |

## Constraints

- `n == height.length`
- `0 <= n <= 3 * 10^4`
- `0 <= height[i] <= 10^5`

## Things to think about before coding

- For a single position, what are the two quantities you need to know to compute its trapped water?
- Can you compute those with one pass using two pointers instead of precomputing two arrays?
- Which pointer should you move when the left bar is shorter than the right bar, and why is that safe?
- What's the difference between "update the running maximum" and "add water"?

Give it a try before scrolling further.

---

## Solution

### Approach

Use two pointers that close in from both ends, maintaining `leftMax` (tallest bar seen from the left) and `rightMax` (tallest seen from the right). At any step, the shorter of the two current bars bounds how much water its side can hold, because the other side is known to be at least that tall. So we can safely resolve that side's contribution without waiting for the rest of the array.

1. Start `left = 0`, `right = n - 1`, `leftMax = 0`, `rightMax = 0`, `water = 0`.
2. Compare `height[left]` and `height[right]`. If the left bar is shorter, process the left side; otherwise process the right side.
3. Processing a side: if the current bar is ≥ that side's running max, update the max (a new wall); otherwise add `max - bar` to `water` (a pit).
4. Move the processed pointer inward by one and repeat until the pointers meet.

**Time:** O(n), one pass. **Space:** O(1).

### TypeScript

```typescript
function trap(height: number[]): number {
  let left = 0;
  let right = height.length - 1;
  let leftMax = 0;
  let rightMax = 0;
  let water = 0;

  while (left < right) {
    const leftHeight = height[left]!;
    const rightHeight = height[right]!;
    if (leftHeight < rightHeight) {
      if (leftHeight >= leftMax) {
        leftMax = leftHeight;
      } else {
        water += leftMax - leftHeight;
      }
      left++;
    } else {
      if (rightHeight >= rightMax) {
        rightMax = rightHeight;
      } else {
        water += rightMax - rightHeight;
      }
      right--;
    }
  }

  return water;
}

// Quick checks
console.log(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
console.log(trap([4, 2, 0, 3, 2, 5])); // 9
console.log(trap([1, 2, 3])); // 0
console.log(trap([3, 2, 1])); // 0
console.log(trap([])); // 0
```

### Rust

```rust
pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0usize;
    let mut right = height.len();
    let mut left_max = 0;
    let mut right_max = 0;
    let mut water = 0;

    while left < right {
        if height[left] < height[right - 1] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right - 1] >= right_max {
                right_max = height[right - 1];
            } else {
                water += right_max - height[right - 1];
            }
            right -= 1;
        }
    }

    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(trap(vec![1, 2, 3]), 0);
        assert_eq!(trap(vec![3, 2, 1]), 0);
        assert_eq!(trap(vec![]), 0);
    }
}
```

### Notes on the two implementations

- Both are two-pointer, O(n) time and O(1) extra space.
- TypeScript uses `right = height.length - 1` and reads `height[right]`; Rust instead sets `right = height.len()` and reads `height[right - 1]`. The Rust style avoids `0 - 1` underflow on empty input without an explicit `is_empty()` guard.
- Rust has no `++`/`--`, so it uses `left += 1` and `right -= 1`.

## Likely interviewer follow-ups

1. **Precomputed arrays:** how would you solve it with `leftMax`/`rightMax` prefix/suffix arrays, and what extra space does that cost?
2. **Monotonic stack:** can you describe a stack-based solution that fills each "basin" when its right wall arrives?
3. **2D version:** how would the problem change if the input were a 2D elevation grid?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Put a pointer at each end of the bars.
2. Keep two running "tallest seen so far" values: one from the left, one from the right.
3. Look at the two bars the pointers sit on. Move the pointer on the **shorter** side.
4. When moving a pointer: if the bar is a new record height, update that side's max (it's a wall). Otherwise, the bar sits in a pit, so add `max - bar` water and keep moving.

## TypeScript, piece by piece

```typescript
let left = 0;
let right = height.length - 1;
let leftMax = 0;
let rightMax = 0;
let water = 0;
```
`left` and `right` are the two pointers, starting at the ends. `leftMax`/`rightMax` track the tallest bar seen so far from each side. `water` accumulates the answer. All use `let` because they will be reassigned inside the loop.

```typescript
const leftHeight = height[left]!;
const rightHeight = height[right]!;
```
We read the two current bars into local `const`s. The `!` is a non-null assertion needed because `noUncheckedIndexedAccess` types `height[left]` as `number | undefined`; inside the loop the indices are always valid, so the assertion is safe.

```typescript
if (leftHeight < rightHeight) {
```
This is the heart of the two-pointer trick. If the left bar is shorter than the right bar, then the left side's water is already determined: no matter what lies between them, the right side is *at least* as tall as `leftHeight`, so `leftMax` is a correct upper bound for the left position. We can therefore advance `left` safely.

```typescript
if (leftHeight >= leftMax) {
  leftMax = leftHeight;
} else {
  water += leftMax - leftHeight;
}
left++;
```
If the current bar is a new left-side record, it becomes the new `leftMax` (it can't hold water — it's a wall). Otherwise the bar is lower than `leftMax`, so it traps `leftMax - leftHeight` units. After resolving the left position, `left++` slides the pointer right. The `else` branch (`rightMax` handling and `right--`) is the exact mirror image.

```typescript
while (left < right) {
```
The loop runs until the two pointers meet. The single cell where they meet is the global maximum region and can never trap water (it has no taller wall on both sides), so stopping there is correct.

## Rust, piece by piece

```rust
let mut left = 0usize;
let mut right = height.len();
```
`left` is `0usize` (unsigned index type). Notice `right` starts at `height.len()` — one past the last index — rather than `len() - 1`. We'll always read `height[right - 1]` instead of `height[right]`, which means an empty `Vec` gives `left = 0, right = 0` and the loop simply never runs (no underflow panic).

```rust
if height[left] < height[right - 1] {
```
The comparison is between the left bar and the *last unprocessed right bar*. Because `left < right` inside the loop, `right` is always at least `1`, so `right - 1` is a valid index — this is the underflow safety the `len()` sentinel buys us.

```rust
if height[left] >= left_max {
    left_max = height[left];
} else {
    water += left_max - height[left];
}
left += 1;
```
Identical logic to TypeScript: a record height updates `left_max`; otherwise we add `left_max - height[left]` to `water`. `left += 1` is Rust's spelling of `left++` (Rust has no increment operators).

```rust
water += right_max - height[right - 1];
right -= 1;
```
The right-side mirror. `right -= 1` is safe for the same reason: `right` is at least `1` whenever we execute this line. `height[right - 1]` indexes a plain `i32` (a copy), no `Option`.

```rust
water
```
The final expression, with no semicolon, is the function's return value — Rust's implicit return. (The function signature promises `-> i32`, and `water` is an `i32`.)

## Trace: `[3,0,2]` (should be `2`)

| Step | left | right | leftMax | rightMax | Action | water |
|---|---|---|---|---|---|---|
| 1 | 0 (`3`) | 2 (`2`) | 0 | 0 | `3 < 2`? no → right side: `2 ≥ 0` so `rightMax = 2`, `right--` | 0 |
| 2 | 0 (`3`) | 1 (`0`) | 0 | 2 | `3 < 0`? no → right side: `0 < 2` so `water += 2 - 0`, `right--` | 2 |

The pointers meet; the single unit between the `3` and `2` walls holds `min(3,2) = 2` units.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Right pointer init | `height.length - 1` | `height.len()` |
| Read right bar | `height[right]` | `height[right - 1]` |
| Increment | `left++` | `left += 1` |
| Decrement | `right--` | `right -= 1` |
| Empty-input safety | natural (`right = -1`) | `len()` sentinel avoids underflow |
| Running max update | `if (x >= max) max = x;` | `if x >= max { max = x; }` |
| Accumulate | `water += …` | `water += …` |
