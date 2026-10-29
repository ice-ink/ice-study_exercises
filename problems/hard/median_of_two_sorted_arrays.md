# Median of Two Sorted Arrays (deepseek generated)

## Problem

Given two sorted arrays `nums1` and `nums2` of sizes `m` and `n`, return the **median** of the two arrays combined. The overall run time complexity must be **O(log(m + n))**.

The median is the middle value of the merged, sorted list: if the combined size is odd, it is the single middle element; if even, it is the average of the two middle elements.

## Examples

| Input | Output | Why |
|---|---|---|
| `nums1 = [1,3]`, `nums2 = [2]` | `2.0` | Merged `[1,2,3]`, middle is `2` |
| `nums1 = [1,2]`, `nums2 = [3,4]` | `2.5` | Merged `[1,2,3,4]`, average of `2` and `3` |
| `nums1 = [0,0]`, `nums2 = [0,0]` | `0.0` | All values equal |
| `nums1 = []`, `nums2 = [1]` | `1.0` | One array may be empty |
| `nums1 = [2]`, `nums2 = []` | `2.0` | The empty array can be either side |

## Constraints

- `nums1.length == m`, `nums2.length == n`
- `0 <= m <= 1000`, `0 <= n <= 1000`
- `1 <= m + n <= 2000`
- `-10^6 <= nums1[i], nums2[i] <= 10^6`

## Things to think about before coding

- Why is merging the two arrays into one and taking the middle too slow here?
- The median is a "split point": can you choose how many elements come from each array in the left half?
- Which array should you binary search over to keep the search small?
- How do you handle a partition that takes nothing from one side (the "empty side")?

Give it a try before scrolling further.

---

## Solution

### Approach

Instead of merging, think of the median as a **partition**. If we split the combined `total = m + n` elements into a left half of size `half = ⌈total / 2⌉` and a right half, the median is determined by the four elements at the boundary: the largest left element and smallest right element from each array. We binary search on how many elements the smaller array contributes to the left half.

1. Ensure `nums1` is the smaller array (swap if needed).
2. Binary search `i` (the number of elements from `nums1` in the left half) over `[0, m]`; the other array contributes `j = half - i`.
3. For each partition, read `left1, right1` (around the cut in `nums1`) and `left2, right2` (around the cut in `nums2`), using `-∞`/`+∞` when a side is empty.
4. If `left1 <= right2 && left2 <= right1`, the partition is correct: return `max(left1, left2)` for odd totals, or the average of `max(left…)` and `min(right…)` for even totals. Otherwise adjust the search range.

**Time:** O(log(min(m, n))). **Space:** O(1).

### TypeScript

```typescript
function findMedianSortedArrays(nums1: number[], nums2: number[]): number {
  if (nums1.length > nums2.length) {
    return findMedianSortedArrays(nums2, nums1);
  }

  const m = nums1.length;
  const n = nums2.length;
  const total = m + n;
  const half = Math.floor((total + 1) / 2);

  let low = 0;
  let high = m;

  while (low <= high) {
    const i = Math.floor((low + high) / 2);
    const j = half - i;

    const left1 = i === 0 ? -Infinity : nums1[i - 1]!;
    const right1 = i === m ? Infinity : nums1[i]!;
    const left2 = j === 0 ? -Infinity : nums2[j - 1]!;
    const right2 = j === n ? Infinity : nums2[j]!;

    if (left1 <= right2 && left2 <= right1) {
      if (total % 2 === 1) {
        return Math.max(left1, left2);
      }
      return (Math.max(left1, left2) + Math.min(right1, right2)) / 2;
    } else if (left1 > right2) {
      high = i - 1;
    } else {
      low = i + 1;
    }
  }

  throw new Error("input arrays are not sorted");
}

// Quick checks
console.log(findMedianSortedArrays([1, 3], [2])); // 2
console.log(findMedianSortedArrays([1, 2], [3, 4])); // 2.5
console.log(findMedianSortedArrays([0, 0], [0, 0])); // 0
console.log(findMedianSortedArrays([], [1])); // 1
console.log(findMedianSortedArrays([2], [])); // 2
```

### Rust

```rust
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let m = a.len();
    let n = b.len();
    let total = m + n;
    let half = (total + 1) / 2;

    let mut low = 0usize;
    let mut high = m;

    while low <= high {
        let i = (low + high) / 2;
        let j = half - i;

        let left1 = if i == 0 { i32::MIN } else { a[i - 1] };
        let right1 = if i == m { i32::MAX } else { a[i] };
        let left2 = if j == 0 { i32::MIN } else { b[j - 1] };
        let right2 = if j == n { i32::MAX } else { b[j] };

        if left1 <= right2 && left2 <= right1 {
            return if total % 2 == 1 {
                left1.max(left2) as f64
            } else {
                (left1.max(left2) + right1.min(right2)) as f64 / 2.0
            };
        } else if left1 > right2 {
            high = i - 1;
        } else {
            low = i + 1;
        }
    }

    unreachable!("input arrays must be sorted")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2.0);
    }
}
```

### Notes on the two implementations

- TypeScript swaps the arrays by recursing with the arguments reversed; Rust swaps once with a tuple destructuring `let (a, b) = …` and never recurses.
- For "empty side" sentinels, TypeScript uses `-Infinity`/`Infinity` while Rust uses `i32::MIN`/`i32::MAX` (integers have no infinity).
- The Rust `return if total % 2 == 1 { … } else { … }` uses the fact that `return <expr>` works with a block expression; the even branch casts the integer sum to `f64` before dividing by `2.0`.

## Likely interviewer follow-ups

1. **Why not merge?** explain concretely why the obvious merge is O(m + n) and fails the required bound.
2. **k-th element:** how would you generalize this to return the `k`-th smallest element of the two arrays?
3. **Sentinel values:** why do `i32::MIN`/`i32::MAX` work here, and what input range would make them unsafe?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Make the first array the smaller one (this keeps the binary search short).
2. Compute the left-half size `half = ⌈total / 2⌉`.
3. Guess `i` = how many of the left-half elements come from the smaller array; the rest `j = half - i` come from the larger one.
4. Look at the four boundary numbers around the cut in each array. If every left number is ≤ every right number, the cut is correct — read off the median. Otherwise slide the cut left or right and repeat.

## TypeScript, piece by piece

```typescript
if (nums1.length > nums2.length) {
  return findMedianSortedArrays(nums2, nums1);
}
```
We force `nums1` to be the shorter array by recursing with the arguments swapped. This guarantees `m <= n`, so the binary search range `[0, m]` is as small as possible — the source of the `O(log min(m,n))` guarantee.

```typescript
const half = Math.floor((total + 1) / 2);
```
`half` is the size of the left partition, which is `⌈total / 2⌉` (the ceiling). The `+ 1` trick makes this work for both parities: for `total = 3` it's `2`, for `total = 4` it's also `2`. `Math.floor` gives the integer division, since JavaScript's `/` on numbers is floating-point.

```typescript
const left1 = i === 0 ? -Infinity : nums1[i - 1]!;
const right1 = i === m ? Infinity : nums1[i]!;
```
`i` is the cut position in `nums1`: indices `0..i-1` go left, `i..m-1` go right. `left1` is the last left element, `right1` the first right element. When the cut is at the edge (`i === 0` or `i === m`), that side is empty, so we substitute `-Infinity`/`Infinity` as a neutral sentinel. The `!` asserts the index is in range (the ternaries already exclude the out-of-range cases).

```typescript
if (left1 <= right2 && left2 <= right1) {
```
This is the correctness check. A valid partition must have the largest element of the left half (`max(left1, left2)`) ≤ the smallest element of the right half (`min(right1, right2)`). Checking the two cross-pairs `left1 <= right2` and `left2 <= right1` guarantees exactly that.

```typescript
if (total % 2 === 1) {
  return Math.max(left1, left2);
}
return (Math.max(left1, left2) + Math.min(right1, right2)) / 2;
```
For an odd total, the median is the single middle element — the bigger of the two left-boundary values. For an even total, it's the average of the max-left and min-right. `total % 2 === 1` tests oddness.

```typescript
} else if (left1 > right2) {
  high = i - 1;
} else {
  low = i + 1;
}
```
If `left1 > right2`, the cut in `nums1` is too far right, so we shrink the high end. Otherwise the cut is too far left and we raise the low end. This is a standard binary-search narrowing. (The final `throw` is unreachable for sorted inputs and only satisfies TypeScript's "all paths return" rule.)

## Rust, piece by piece

```rust
let (a, b) = if nums1.len() <= nums2.len() {
    (nums1, nums2)
} else {
    (nums2, nums1)
};
```
Rust's `if` is an expression, so we can bind a whole tuple `(a, b)` to its result. This is a single swap (no recursion): whichever of the two inputs is shorter becomes `a`. The values are **moved** into `a` and `b`; after this line `nums1`/`nums2` are gone.

```rust
let half = (total + 1) / 2;
```
Because `total` is a `usize`, `/` is **integer division** (truncating), so this computes `⌈total / 2⌉` directly — no `floor` call needed. In Rust, dividing integers always yields an integer, unlike JavaScript's `/`.

```rust
let left1 = if i == 0 { i32::MIN } else { a[i - 1] };
let right1 = if i == m { i32::MAX } else { a[i] };
```
Same boundary logic as TypeScript, but the sentinels are `i32::MIN` and `i32::MAX` because Rust `i32` has no infinity. These work since problem values are within `[-10^6, 10^6]`. Indexing `a[i - 1]` returns an `i32` (a copy), not an `Option`, so no `unwrap`.

```rust
(left1.max(left2) + right1.min(right2)) as f64 / 2.0
```
`left1.max(left2)` and `right1.min(right2)` are `i32` methods for max/min. The `as f64` converts the integer **sum** to a float; the parentheses make the cast apply to the sum before dividing by `2.0` (a float literal). Mixing `f64 / 2.0` is what produces a fractional median.

```rust
} else if left1 > right2 {
    high = i - 1;
} else {
    low = i + 1;
}
```
`high = i - 1` is safe from underflow: `left1 > right2` can only be true when `i > 0` (if `i == 0`, `left1` is `i32::MIN`, which is never greater than anything). This is the same `usize` guard pattern as the number-of-islands solution.

```rust
unreachable!("input arrays must be sorted")
```
`unreachable!` is a macro that panics with a message if ever reached. For correctly sorted inputs the `while` loop always returns from the found branch, so this line is never hit — but it lets Rust know the function always produces a value (the macro's type is `!`, which coerces to any return type).

## Trace: `nums1 = [1,3]`, `nums2 = [2]` (should be `2`)

Since `len(nums1) = 2 > len(nums2) = 1`, swap → `a = [2]`, `b = [1,3]`, so `m = 1`, `n = 2`, `total = 3`, `half = 2`.

| Step | i | j | left1 | right1 | left2 | right2 | Outcome |
|---|---|---|---|---|---|---|---|
| 1 | 0 | 2 | -∞ | 2 | 3 | +∞ | `left2(3) > right1(2)` → `low = 1` |
| 2 | 1 | 1 | 2 | +∞ | 1 | 3 | valid; odd total → `max(2, 1) = 2` |

The left half is `[1, 2]`, the right half is `[3]`, and the middle value is `2`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| "Smaller array" swap | recursive call | tuple destructuring |
| Ceiling of `n/2` | `Math.floor((total + 1) / 2)` | `(total + 1) / 2` |
| Empty-left sentinel | `-Infinity` | `i32::MIN` |
| Empty-right sentinel | `Infinity` | `i32::MAX` |
| Max of two | `Math.max(a, b)` | `a.max(b)` |
| Odd test | `total % 2 === 1` | `total % 2 == 1` |
| Int → float | automatic (`/ 2`) | `as f64 / 2.0` |
