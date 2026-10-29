# Reverse String (deepseek generated)

## Problem

Given an array of characters `s`, reverse the array **in place** so the characters appear in the opposite order. The function returns nothing; it must modify the input array directly and must not allocate any extra array or other data structure that scales with the input size.

## Examples

| Input | Output | Why |
|---|---|---|
| `["h","e","l","l","o"]` | `["o","l","l","e","h"]` | Whole array reversed |
| `["H","a","n","n","a","h"]` | `["h","a","n","n","a","H"]` | Reversed, case preserved |
| `["a","b"]` | `["b","a"]` | Even-length swap |
| `["a"]` | `["a"]` | Single element is its own reverse |
| `[]` | `[]` | Nothing to reverse |

## Constraints

- `0 <= s.length <= 10^5`
- `s[i]` is a printable ASCII character.
- You must do this in-place with **O(1) extra space**.

## Things to think about before coding

- What if you swap the first and last element, then move both toward the middle?
- When exactly do the two pointers need to stop so you don't swap everything back into the original order?
- How do you swap two values without losing one of them?

Give it a try before scrolling further.

---

## Solution

### Approach

Put one pointer at the front and one at the back, swap their values, then walk them toward each other until they meet or cross.

1. Set `left = 0` and `right = s.length - 1`.
2. While `left < right`, swap `s[left]` and `s[right]`.
3. Move `left` up by one and `right` down by one.
4. When `left >= right`, every pair has been swapped, so stop.

**Time:** O(n), each element is touched once. **Space:** O(1), only two index variables and a temporary swap value.

### TypeScript

```typescript
function reverseString(s: string[]): void {
  let left = 0;
  let right = s.length - 1;

  while (left < right) {
    const tmp = s[left]!;
    s[left] = s[right]!;
    s[right] = tmp;
    left++;
    right--;
  }
}

// Quick checks
const s1 = ["h", "e", "l", "l", "o"];
reverseString(s1);
console.log(s1); // ["o","l","l","e","h"]

const s2 = ["H", "a", "n", "n", "a", "h"];
reverseString(s2);
console.log(s2); // ["h","a","n","n","a","H"]

const s3 = ["a"];
reverseString(s3);
console.log(s3); // ["a"]

const s4: string[] = [];
reverseString(s4);
console.log(s4); // []
```

### Rust

```rust
pub fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0usize;
    let mut right = s.len();

    while left < right {
        right -= 1;
        s.swap(left, right);
        left += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

        let mut mixed = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut mixed);
        assert_eq!(mixed, vec!['h', 'a', 'n', 'n', 'a', 'H']);

        let mut single = vec!['a'];
        reverse_string(&mut single);
        assert_eq!(single, vec!['a']);

        let mut empty: Vec<char> = Vec::new();
        reverse_string(&mut empty);
        assert!(empty.is_empty());
    }
}
```

### Notes on the two implementations

- TypeScript has no tuple-swap in this form for an array write target under strict indexing, so it uses a temporary variable (`tmp`) to hold one value during the swap.
- Rust uses the built-in `Vec::swap`, which swaps two elements by index in place — no temporary variable is needed, and it is safe and idiomatic.
- Both start `right` from the length (Rust) versus `length - 1` (TypeScript), which is why the Rust loop decrements `right` *before* the swap.

## Likely interviewer follow-ups

1. **Recursive version:** how would you write this recursively, and what extra space would that cost (the call stack)?
2. **String, not array:** if the input were an immutable string, why can't you do this in place, and what would you build instead?
3. **Reverse a sub-range:** how would you extend this to reverse only `s[i..=j]` and use it to rotate an array?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Put one marker (`left`) at the start of the array and one (`right`) at the end.
2. Swap the two characters those markers point at.
3. Move `left` one step right and `right` one step left.
4. Stop when `left` is no longer strictly less than `right` — everything has been swapped.

## TypeScript, piece by piece

```typescript
function reverseString(s: string[]): void {
```
`s: string[]` means "an array of strings." The `: void` return type says this function returns nothing — it changes the array the caller passed in. (An array is a *reference*, so mutating its elements is visible outside the function.)

```typescript
let left = 0;
let right = s.length - 1;
```
Two index variables. `s.length - 1` is the index of the last element, because indexes start at 0. `let` (not `const`) is used because these will be reassigned each loop.

```typescript
while (left < right) {
```
Keep going while the two markers haven't met or crossed. When `left === right`, both point at the same middle element, which doesn't need to move; when `left > right`, we'd be swapping pairs back again.

```typescript
const tmp = s[left]!;
s[left] = s[right]!;
s[right] = tmp;
```
A three-line swap. `tmp` holds the value at `left` so we don't lose it when we overwrite `s[left]`. The `!` is a **non-null assertion**: with `noUncheckedIndexedAccess`, `s[left]` is typed `string | undefined`, but we know the index is valid inside the loop, so `!` tells TypeScript "trust me, this is a `string`."

```typescript
left++;
right--;
```
Move the markers inward. `++` and `--` add/subtract one.

## Rust, piece by piece

```rust
pub fn reverse_string(s: &mut Vec<char>) {
```
- `&mut Vec<char>` is a **mutable borrow** of a vector of characters. The `&mut` means "I'm allowed to change what I'm looking at," which is required to mutate `s`.
- There's no return type annotation because the function returns nothing — Rust calls that the unit type `()`, and it's omitted.

```rust
let mut left = 0usize;
let mut right = s.len();
```
`0usize` explicitly types `left` as `usize`, an unsigned integer used for lengths and indexes. `right` starts at `s.len()` (the number of elements), which is one *past* the last index — we decrement before using it. `mut` is required because both change later.

```rust
while left < right {
    right -= 1;
    s.swap(left, right);
    left += 1;
}
```
- `right -= 1` moves `right` down to the last unprocessed index first. This is safe because `left < right` guarantees `right >= 1`, so it can't underflow (usize can't go negative).
- `s.swap(left, right)` swaps the two elements in place — the idiomatic Rust equivalent of the three-line temp swap.
- `left += 1` moves `left` up. Rust has no `++`, so `+= 1` is the idiom.

```rust
s.swap(left, right)
```
`Vec::swap` takes two indexes and exchanges their values. It's a built-in method, so you don't hand-roll the temp variable, and it panics only if either index is out of bounds (which the loop conditions prevent).

```rust
let mut empty: Vec<char> = Vec::new();
reverse_string(&mut empty);
assert!(empty.is_empty());
```
In the test module, `Vec::<char>::new()` builds an empty vector. We pass `&mut empty` to hand over a mutable borrow. `assert!(empty.is_empty())` checks the empty case still ends empty — note `reverse_string`'s `while left < right` with `left = 0, right = 0` never runs, which is exactly what we want.

## Trace: `["h","e","l","l","o"]` (should be `["o","l","l","e","h"]`)

| Step | `left` | `right` | Action | Array after |
|---|---|---|---|---|
| start | 0 | 4 | — | `h e l l o` |
| 1 | 0 | 4 | swap(0,4) | `o e l l h` |
| 2 | 1 | 3 | swap(1,3) | `o l l e h` |
| 3 | 2 | 2 | `left < right` is false, stop | `o l l e h` |

The middle `l` never moves, which is correct — it's the center of an odd-length array.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Mutable borrow of an array | (arrays are always references) | `&mut Vec<char>` |
| Index type | `number` | `usize` |
| Swap two elements | 3 lines with `tmp` | `s.swap(i, j)` |
| Increment | `left++` | `left += 1` |
| Decrement | `right--` | `right -= 1` |
| Start of "last element" | `s.length - 1` | `s.len()` then `right -= 1` |
| Return nothing | `: void` | omitted (unit type `()`) |
