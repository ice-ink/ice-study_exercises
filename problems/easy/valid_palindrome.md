# valid palindrome (claude generated)

## problem

# Exercise 2: Valid Palindrome (Easy)

## Problem

Given a string `s`, return `true` if it reads the same forwards and backwards, **ignoring case and any character that isn't a letter or digit**. Otherwise return `false`.

## Examples

| Input | Output | Why |
|---|---|---|
| `"racecar"` | `true` | Same both ways |
| `"A man, a plan, a canal: Panama"` | `true` | Cleaned: `amanaplanacanalpanama` |
| `"hello"` | `false` | `h` ≠ `o` |
| `"Ab"` | `false` | `a` ≠ `b` |
| `" "` | `true` | Nothing left after cleaning, which counts as a palindrome |
| `""` | `true` | Empty is a palindrome |

## Constraints

- `0 <= s.length <= 10^5`
- `s` contains only printable ASCII characters.

## Things to think about

- The easy approach is to build a cleaned, lowercase copy of the string and compare it to its reverse. What does that cost in space?
- Can you do it with **no extra copy**, using two positions that walk toward each other?
- What should happen when one of those positions lands on a punctuation mark?

Give it a try before scrolling further.

---

## Solution

### Approach: two pointers

Put one pointer at the start (`left`) and one at the end (`right`), then repeat until they meet:
1. If `left` points at a non-alphanumeric character, move it right and continue.
2. If `right` points at a non-alphanumeric character, move it left and continue.
3. Otherwise compare the two characters (ignoring case). If they differ, return `false`. If they match, move both pointers inward.

If the pointers cross without finding a mismatch, return `true`.

**Time:** O(n). **Space:** O(1), since no copy is made.

### TypeScript

```typescript
function isPalindrome(s: string): boolean {
  // Regex test: is this character a letter or digit? (the i makes it case-insensitive)
  const isAlnum = (c: string): boolean => /[a-z0-9]/i.test(c);

  let left = 0;
  let right = s.length - 1;

  while (left < right) {
    if (!isAlnum(s[left])) {
      left++;
      continue;
    }
    if (!isAlnum(s[right])) {
      right--;
      continue;
    }
    if (s[left].toLowerCase() !== s[right].toLowerCase()) {
      return false;
    }
    left++;
    right--;
  }

  return true;
}

console.log(isPalindrome("A man, a plan, a canal: Panama")); // true
console.log(isPalindrome("hello"));                          // false
console.log(isPalindrome(" "));                              // true
```

### Rust

```rust
pub fn is_palindrome(s: &str) -> bool {
    // Input is ASCII only, so we can work with raw bytes (u8) safely.
    let bytes = s.as_bytes();

    // Guard: for an empty string, len() - 1 would underflow (usize can't be negative).
    if bytes.is_empty() {
        return true;
    }

    let mut left: usize = 0;
    let mut right: usize = bytes.len() - 1;

    while left < right {
        if !bytes[left].is_ascii_alphanumeric() {
            left += 1;
            continue;
        }
        if !bytes[right].is_ascii_alphanumeric() {
            right -= 1;
            continue;
        }
        if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome(" "));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("Ab"));
    }
}
```

### Syntax notes (since last time's were the sticking point)

- **TypeScript `continue`** skips the rest of the loop body and jumps to the next iteration. That's how we "skip" punctuation without nesting `if`s.
- **Rust `+=` / `-=`** are used because Rust has no `++` or `--`.
- **Rust `usize`** is an unsigned integer, so it can never go negative. That's why the empty-string guard exists: `0 - 1` would panic. Inside the loop, `right -= 1` is safe because `left < right` guarantees `right >= 1`.
- **`s.as_bytes()`** views the string as a slice of `u8` values, which lets us index with `bytes[left]`. Rust doesn't allow `s[left]` on a `&str`, because UTF-8 characters can vary in width.
- **`is_ascii_alphanumeric()` and `to_ascii_lowercase()`** are built-in methods on `u8`, so no regex is needed.

## Likely interviewer follow-ups

1. **Simpler version first:** how would you solve it by building a cleaned string and comparing it to its reverse? What's the tradeoff?
2. **Unicode:** if the input could contain non-ASCII letters like `é`, what would break in the Rust version?
3. **Almost-palindrome:** how would you check whether the string is a palindrome after deleting **at most one** character? (This is a well-known step up in difficulty.)

When you're ready, send me your attempt or any questions, or ask for the next exercise.

## Syntax Explanation

## The core idea in plain steps

1. Put one marker at the start of the string and one at the end.
2. If a marker is sitting on punctuation or a space, slide it inward and look again.
3. If both markers are on real characters, compare them (ignoring case). If they differ, return false. If they match, move both inward.
4. When the markers meet or cross, nothing mismatched, so return true.

## TypeScript, piece by piece

```typescript
const isAlnum = (c: string): boolean => /[a-z0-9]/i.test(c);
```
This line has three ideas:
- **Arrow function:** `(c: string): boolean => ...` is a short way to write a function. It takes `c` (a string), returns a boolean, and whatever follows `=>` is the return value. It's equivalent to `function isAlnum(c: string): boolean { return ...; }`.
- **Regex literal:** `/[a-z0-9]/i` is a pattern between slashes. `[a-z0-9]` means "any one character that is a lowercase letter or a digit." The trailing `i` flag makes it case-insensitive, so uppercase letters match too.
- **`.test(c)`** runs the pattern against `c` and returns `true` or `false`. Since `c` is a single character, we don't need to worry about the pattern matching in the middle of a longer string.

```typescript
let left = 0;
let right = s.length - 1;
```
`let` declares a variable you can change later. (Use `const` for values that never change.) `s.length - 1` is the index of the last character, because indexes start at 0. TypeScript infers that both are numbers, so no type annotation is needed.

```typescript
while (left < right) {
```
Keep looping as long as the markers haven't met or crossed. When `left === right`, they're on the same character (the middle), which trivially matches itself, so there's nothing left to check.

```typescript
if (!isAlnum(s[left])) {
  left++;
  continue;
}
```
- `s[left]` gets the character at that index.
- `!` flips true/false, so `!isAlnum(...)` reads "is NOT a letter or digit."
- `left++` adds 1 to `left`.
- `continue` abandons the rest of this loop pass and jumps back to the `while` check. Without it, the code would fall through and compare a punctuation mark.

The `right` block is the mirror image, using `right--` to move left.

```typescript
if (s[left].toLowerCase() !== s[right].toLowerCase()) {
  return false;
}
```
Both characters are real letters or digits at this point. `.toLowerCase()` turns `"A"` into `"a"` so the comparison ignores case. `!==` means "not equal." A mismatch means it isn't a palindrome, so we exit immediately.

```typescript
left++;
right--;
return true;
```
A match, so move both markers inward. If the loop ends without ever returning `false`, the answer is `true`.

## Rust, piece by piece

```rust
let bytes = s.as_bytes();
```
Rust won't let you write `s[left]` on a `&str`, because UTF-8 characters can take 1 to 4 bytes, so "the Nth character" isn't a simple lookup. `.as_bytes()` gives a `&[u8]`, a read-only view of the string as raw bytes (numbers 0 to 255), which you *can* index. This is safe here because the problem guarantees ASCII, where one character is exactly one byte.

```rust
if bytes.is_empty() {
    return true;
}
```
This guard exists because of the next line. An explicit `return` with a semicolon is fine here because it's an early exit. Only the *final* expression of the function drops the `return`.

```rust
let mut left: usize = 0;
let mut right: usize = bytes.len() - 1;
```
- `mut` is needed because we change these later (Rust variables are immutable by default).
- `: usize` is an unsigned integer type used for indexes and lengths. "Unsigned" means it can never be negative.
- That's why the guard matters: for an empty string, `bytes.len() - 1` is `0 - 1`, which can't be represented in an unsigned type and panics (in debug builds).

```rust
while left < right {
```
Same as TypeScript, but with no parentheses around the condition. Rust doesn't require them.

```rust
if !bytes[left].is_ascii_alphanumeric() {
    left += 1;
    continue;
}
```
- `bytes[left]` is a `u8`.
- `.is_ascii_alphanumeric()` is a built-in method on `u8` that returns `true` for `a-z`, `A-Z`, and `0-9`. No regex needed.
- `!` negates, just like in TypeScript.
- `left += 1` is Rust's version of `left++`, since Rust has no `++` or `--`.
- `continue` works the same as in TypeScript.

```rust
if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
    return false;
}
```
`.to_ascii_lowercase()` on a `u8` returns the lowercase version, so `b'A'` becomes `b'a'`. Then `!=` compares the two bytes.

```rust
left += 1;
right -= 1;
```
The `right -= 1` is safe from underflow because the loop condition `left < right` guarantees `right` is at least 1.

```rust
true
```
No `return`, no semicolon. This is the function's final expression, so it's the return value.

### The test module

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("hello"));
    }
}
```
- `#[cfg(test)]` means "only compile this when running tests."
- `mod tests` creates a nested module.
- `use super::*;` imports everything from the parent module, so `is_palindrome` is visible.
- `#[test]` marks a function as a test. Run them with `cargo test`.
- `assert!(cond)` fails the test if `cond` is false. `assert!(!cond)` checks that it's false.

## Trace: `"A b,a"` (should be `true`)

Indexes: `A`=0, space=1, `b`=2, `,`=3, `a`=4.

| Step | left | right | What happens | After |
|---|---|---|---|---|
| 1 | 0 (`A`) | 4 (`a`) | Both alphanumeric, `a` = `a`, match | left=1, right=3 |
| 2 | 1 (space) | 3 (`,`) | `left` is a space, skip it | left=2 |
| 3 | 2 (`b`) | 3 (`,`) | `left` is fine, `right` is a comma, skip it | right=2 |
| 4 | 2 | 2 | `left < right` is false, loop ends | return `true` |

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Increment | `left++` | `left += 1` |
| Decrement | `right--` | `right -= 1` |
| Character at index | `s[left]` | `bytes[left]` (after `as_bytes()`) |
| Alphanumeric check | regex `/[a-z0-9]/i.test(c)` | `.is_ascii_alphanumeric()` |
| Lowercase | `.toLowerCase()` | `.to_ascii_lowercase()` |
| Not equal | `!==` | `!=` |
| Index type | `number` | `usize` (unsigned, can't go below 0) |
| Final return | `return true;` | `true` (no semicolon) |

If any part is still unclear, tell me which one (the regex, `usize` and underflow, `as_bytes`, or something else) and I'll go deeper with smaller examples. Or if you're ready, try writing your own version and send it over for review.