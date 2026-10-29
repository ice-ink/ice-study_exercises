# Longest Valid Parentheses (deepseek generated)

## Problem

Given a string `s` containing only the characters `'('` and `')'`, return the length of the **longest valid (well-formed) parentheses substring**.

A substring is valid if every `'('` has a matching later `')'` and every `')'` has a matching earlier `'('`, with correct nesting. For example, in `")()())"` the longest valid substring is `"()()"`, of length `4`.

## Examples

| Input | Output | Why |
|---|---|---|
| `"(()"` | `2` | The substring `"()"` (indices 1–2) is valid |
| `")()())"` | `4` | `"()()"` (indices 1–4) is the longest |
| `""` | `0` | Empty string has no valid substring |
| `"()(()"` | `2` | `"()"` at the start; the rest can't close |
| `"()()"` | `4` | The whole string is valid |

## Constraints

- `0 <= s.length <= 3 * 10^4`
- `s[i]` is `'('` or `')'`

## Things to think about before coding

- If you store *indices* (not just characters) on a stack, what can the distance between indices tell you?
- What role does a "sentinel" at the bottom of the stack play?
- When a `')'` has nothing to match, why is that position the new "start" boundary?
- What is the meaning of `currentIndex - stackTopIndex` after a successful match?

Give it a try before scrolling further.

---

## Solution

### Approach

Keep a stack of **indices**, initialized with a sentinel `-1` that marks the position just before any valid substring could start. On `'('`, push its index. On `')'`, pop; if the stack becomes empty, the `')'` was unmatched, so push *its* index as the new sentinel (the earliest possible start of a future valid substring). Otherwise, the current top is the last unmatched position, and `i - stack[top]` is the length of the currently valid substring — track the maximum.

1. Start with `stack = [-1]` and `maxLen = 0`.
2. For each index `i`:
   - If `s[i]` is `'('`, push `i`.
   - Else pop. If the stack is empty, push `i`; otherwise `maxLen = max(maxLen, i - stack[top])`.
3. Return `maxLen`.

**Time:** O(n), one pass. **Space:** O(n) for the stack.

### TypeScript

```typescript
function longestValidParentheses(s: string): number {
  const stack: number[] = [-1];
  let maxLen = 0;

  for (let i = 0; i < s.length; i++) {
    if (s[i] === "(") {
      stack.push(i);
    } else {
      stack.pop();
      if (stack.length === 0) {
        stack.push(i);
      } else {
        maxLen = Math.max(maxLen, i - stack[stack.length - 1]!);
      }
    }
  }

  return maxLen;
}

// Quick checks
console.log(longestValidParentheses("(()")); // 2
console.log(longestValidParentheses(")()())")); // 4
console.log(longestValidParentheses("")); // 0
console.log(longestValidParentheses("()(()")); // 2
console.log(longestValidParentheses("()()")); // 4
```

### Rust

```rust
pub fn longest_valid_parentheses(s: &str) -> i32 {
    let mut stack: Vec<i32> = vec![-1];
    let mut max_len = 0;

    for (i, ch) in s.chars().enumerate() {
        let i = i as i32;
        if ch == '(' {
            stack.push(i);
        } else {
            stack.pop();
            if stack.is_empty() {
                stack.push(i);
            } else {
                max_len = max_len.max(i - stack[stack.len() - 1]);
            }
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(longest_valid_parentheses("(()"), 2);
        assert_eq!(longest_valid_parentheses(")()())"), 4);
        assert_eq!(longest_valid_parentheses(""), 0);
        assert_eq!(longest_valid_parentheses("()(()"), 2);
        assert_eq!(longest_valid_parentheses("()()"), 4);
    }
}
```

### Notes on the two implementations

- The trick is identical in both: the stack holds *indices*, and `i - stackTop` measures a valid span without a separate DP array.
- TypeScript indexes the string directly with `s[i]`; Rust iterates `s.chars().enumerate()` and casts the `usize` index to `i32` so it can share the stack with the `-1` sentinel.
- In Rust, reading the last element uses `stack[stack.len() - 1]` (a direct index, valid because the branch guarantees a non-empty stack) rather than an `Option`-returning `pop()`.

## Likely interviewer follow-ups

1. **DP version:** how would you compute the same result with a `dp` array, and what would `dp[i]` represent?
2. **No stack:** can you solve it with two counters (left-to-right and right-to-left passes) in O(1) space? What does that approach get wrong in a single pass?
3. **Counting substrings:** how would you count the *number* of distinct valid substrings instead of the longest length?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Put `-1` on the stack as a fake "last unmatched position" before the string starts.
2. For each character and its index `i`:
   - `'('` → push `i`.
   - `')'` → pop the top. If the stack is now empty, this `')'` had no match, so push `i` as the new fake start. Otherwise the top is the last unmatched index, and `i - top` is a valid length — remember the biggest.
3. Return the biggest length seen.

## TypeScript, piece by piece

```typescript
const stack: number[] = [-1];
```
We store *indices*, not characters. The initial `-1` is a **sentinel**: it marks the boundary just before index `0`, so the very first valid substring can be measured as `i - (-1) = i + 1` without special-casing.

```typescript
for (let i = 0; i < s.length; i++) {
  if (s[i] === "(") {
    stack.push(i);
  }
```
A `for` loop with an explicit counter (rather than `for…of`) is used because we need the **index** `i`, not just the character. Every `'('` pushes its index onto the stack, waiting for a matching `')'`. (`s[i]` is `string | undefined` under `noUncheckedIndexedAccess`, but the comparison `=== "("` is allowed.)

```typescript
} else {
  stack.pop();
  if (stack.length === 0) {
    stack.push(i);
  }
```
On a `')'`, we first pop — this either removes a matching `'('` or, if the top was a sentinel, removes it. If the stack is then empty, the `')'` had no opener, so it becomes the new sentinel (pushed as its own index). That resets the "valid substring starts here" boundary to `i`, which is the crucial idea.

```typescript
} else {
  maxLen = Math.max(maxLen, i - stack[stack.length - 1]!);
}
```
If the stack is *not* empty after popping, the current top is the last unmatched index, and `i - top` is the length of the valid substring ending at `i`. `Math.max` keeps the largest. The `!` asserts `stack[stack.length - 1]` is defined — provably safe because we just confirmed `stack.length > 0`.

```typescript
return maxLen;
```
After the loop, `maxLen` holds the longest valid span. We return it explicitly (TypeScript requires `return`).

## Rust, piece by piece

```rust
let mut stack: Vec<i32> = vec![-1];
let mut max_len = 0;
```
`vec![-1]` creates a `Vec` with a single element, the sentinel `-1`. The type is `Vec<i32>` so it can hold both the sentinel and the string indices (which we'll cast to `i32`). `mut` is required for both bindings since they change.

```rust
for (i, ch) in s.chars().enumerate() {
    let i = i as i32;
```
`.chars()` turns the `&str` into an iterator of `char`s, and `.enumerate()` pairs each with its index. The index `i` arrives as `usize`; we cast it with `i as i32` so it can live in the same `Vec<i32>` as `-1`. (The constraint `s.length <= 3 * 10^4` means this cast never overflows `i32`.)

```rust
if ch == '(' {
    stack.push(i);
} else {
    stack.pop();
```
Exactly as in TypeScript: openers push their index, closers pop. `stack.pop()` returns an `Option<i32>`, but we ignore the value — we only care whether the stack is empty afterward, which is why there's no `unwrap`.

```rust
if stack.is_empty() {
    stack.push(i);
} else {
    max_len = max_len.max(i - stack[stack.len() - 1]);
}
```
If the pop emptied the stack, `i` becomes the new sentinel. Otherwise `stack[stack.len() - 1]` reads the top via **direct indexing** (it returns an `i32`, not an `Option`, and is valid because the `else` branch means the stack is non-empty). `max_len.max(…)` returns the larger of the two.

```rust
max_len
```
The final expression with no semicolon is the return value (implicit return). `max_len` is an `i32`, matching the `-> i32` signature.

## Trace: `")()())"` (should be `4`)

| Step | i | char | stack after | maxLen |
|---|---|---|---|---|
| 1 | 0 | `)` | `[0]` (popped sentinel, pushed 0) | 0 |
| 2 | 1 | `(` | `[0,1]` | 0 |
| 3 | 2 | `)` | `[0]` (popped 1); `2 - 0 = 2` | 2 |
| 4 | 3 | `(` | `[0,3]` | 2 |
| 5 | 4 | `)` | `[0]` (popped 3); `4 - 0 = 4` | 4 |
| 6 | 5 | `)` | `[5]` (popped 0, pushed 5) | 4 |

The longest span runs from index 1 to 4 (`"()()"`), length `4`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Stack with sentinel | `const stack: number[] = [-1]` | `let mut stack: Vec<i32> = vec![-1]` |
| Loop with index | `for (let i = 0; i < s.length; i++)` | `for (i, ch) in s.chars().enumerate()` |
| Read character | `s[i]` | `ch` (from the iterator) |
| Cast index | none needed | `let i = i as i32` |
| Is empty | `stack.length === 0` | `stack.is_empty()` |
| Top element | `stack[stack.length - 1]!` | `stack[stack.len() - 1]` |
| Max of two | `Math.max(a, b)` | `a.max(b)` |
