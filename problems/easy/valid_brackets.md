# valid brackets problem (claude generated)

# Exercise 1: Valid Brackets (Easy)

## Problem

Given a string `s` containing only the characters `(`, `)`, `[`, `]`, `{`, and `}`, determine whether it is **valid**.

A string is valid if:
1. Every opening bracket is closed by the same type of bracket.
2. Brackets are closed in the correct order.
3. Every closing bracket has a matching opening bracket before it.

## Examples

| Input | Output | Why |
|---|---|---|
| `"()"` | `true` | Simple match |
| `"()[]{}"` | `true` | Three matched pairs in sequence |
| `"{[]}"` | `true` | Properly nested |
| `"(]"` | `false` | Wrong closing type |
| `"([)]"` | `false` | Closed in the wrong order |
| `"(("` | `false` | Unclosed brackets |
| `")"` | `false` | Closing bracket with nothing to match |
| `""` | `true` | Empty string is valid |

## Constraints

- `0 <= s.length <= 10^4`
- `s` consists only of the six bracket characters above.

## Things to think about before coding

- What data structure naturally models "the most recent unclosed thing"?
- What edge cases can make a naive approach fail? (Try `")("` and `"(("`.)
- What's the best time and space complexity you can achieve?

Give it a try before scrolling further.

---

## Solution

### Approach

Use a **stack**. Walk through the string:
- On an opening bracket, push it.
- On a closing bracket, pop the top of the stack. If the stack is empty or the popped bracket doesn't match, return `false`.
- At the end, the string is valid only if the stack is empty (no unclosed openers).

**Time:** O(n), one pass. **Space:** O(n) worst case, e.g. `"((((("`.

### TypeScript

```typescript
function isValid(s: string): boolean {
  const closeToOpen: Record<string, string> = {
    ")": "(",
    "]": "[",
    "}": "{",
  };
  const stack: string[] = [];

  for (const ch of s) {
    if (ch in closeToOpen) {
      // pop() returns undefined on an empty stack, which never matches
      if (stack.pop() !== closeToOpen[ch]) return false;
    } else {
      stack.push(ch);
    }
  }

  return stack.length === 0;
}

// Quick checks
console.log(isValid("()[]{}")); // true
console.log(isValid("([)]"));   // false
console.log(isValid("(("));     // false
console.log(isValid(""));       // true
```

### Rust

```rust
pub fn is_valid(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                let expected = match ch {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                // pop() returns None on an empty stack, which never matches
                if stack.pop() != Some(expected) {
                    return false;
                }
            }
            _ => {} // constraints say this can't happen
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_valid("()[]{}"));
        assert!(is_valid("{[]}"));
        assert!(is_valid(""));
        assert!(!is_valid("([)]"));
        assert!(!is_valid("(("));
        assert!(!is_valid(")"));
    }
}
```

### Notes on the two implementations

- Both rely on the same trick: popping from an empty stack yields "nothing" (`undefined` / `None`), which can never equal a real bracket, so the empty-stack case is handled without a separate check.
- In Rust, `Vec<char>` works fine here. For raw performance on ASCII-only input you could iterate over `s.bytes()` and use `Vec<u8>`, but `char` is clearer for an interview.

## Likely interviewer follow-ups

1. **Early exit optimization:** if `s.length` is odd, return `false` immediately. Why does this work?
2. **Space optimization:** if only one bracket type existed, you could replace the stack with a single counter. Why does that break with multiple types?
3. **Extension:** how would you return the *index* of the first offending bracket instead of a boolean?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

Here is the logic, with no language-specific syntax:

1. Make an empty stack.
2. For each character:
   - If it's an **opener**, push it.
   - If it's a **closer**, pop the stack and check that the popped item is the matching opener. If not, return false.
3. At the end, return true only if the stack is empty.

## TypeScript, piece by piece

```typescript
function isValid(s: string): boolean {
```
`s: string` means the parameter `s` must be a string. The `: boolean` after the parentheses is the return type.

```typescript
const closeToOpen: Record<string, string> = {
  ")": "(",
  "]": "[",
  "}": "{",
};
```
This is a lookup table (like a dictionary). `Record<string, string>` is a type meaning "an object whose keys are strings and whose values are strings." Given a *closer*, it tells you which *opener* it needs: `closeToOpen[")"]` gives `"("`.

```typescript
const stack: string[] = [];
```
An empty array, typed as "array of strings." We use it as a stack: `push` adds to the end and `pop` removes from the end.

```typescript
for (const ch of s) {
```
Loops over the string one character at a time. `ch` is the current character. (`for...of` gives you the values. Its cousin `for...in` gives you indexes or keys, so don't mix them up.)

```typescript
if (ch in closeToOpen) {
```
The `in` operator checks whether `ch` is a **key** in the object. So this reads as "is this character one of `)`, `]`, `}`?" If it is, it's a closer.

```typescript
if (stack.pop() !== closeToOpen[ch]) return false;
```
This line does a lot:
- `stack.pop()` removes and returns the last item. If the stack is empty, it returns `undefined` instead of crashing.
- `closeToOpen[ch]` looks up the opener this closer needs.
- `!==` means "not equal."
- If the two differ, return `false`. This covers both a wrong match like `(]` and an empty stack like `)`, because `undefined` never equals a bracket.

```typescript
} else {
  stack.push(ch);
}
```
If it wasn't a closer, it must be an opener, so push it. (The constraints guarantee only brackets appear.)

```typescript
return stack.length === 0;
```
After the loop, any leftover items are unclosed openers. `===` is strict equality, and the expression evaluates directly to `true` or `false`.

## Rust, piece by piece

```rust
pub fn is_valid(s: &str) -> bool {
```
- `pub` makes the function visible outside the module.
- `&str` is a borrowed string slice, meaning "I'm looking at a string but don't own it." This is the usual type for string parameters.
- `-> bool` is the return type.

```rust
let mut stack: Vec<char> = Vec::new();
```
- `Vec<char>` is a growable array of characters.
- `mut` is required because Rust variables are **immutable by default**. Without `mut`, `push` and `pop` won't compile.

```rust
for ch in s.chars() {
```
`.chars()` turns the string into an iterator over its characters, and `ch` is each one in turn.

```rust
match ch {
    '(' | '[' | '{' => stack.push(ch),
    ')' | ']' | '}' => { ... }
    _ => {}
}
```
`match` is like a more powerful `switch`:
- Each line is `pattern => what to do`.
- `|` means "or", so `'(' | '[' | '{'` matches any of those three.
- `_` is the wildcard that matches everything else. `{}` is an empty block, meaning "do nothing."
- Single quotes (`'('`) are a single `char`. Double quotes are strings.

```rust
let expected = match ch {
    ')' => '(',
    ']' => '[',
    _ => '{',
};
```
In Rust, `match` is an **expression**: it produces a value, so you can assign it to a variable. This is the lookup table, written as a match. The final `_` arm covers `}`, since we already know `ch` is one of the three closers.

```rust
if stack.pop() != Some(expected) {
    return false;
}
```
This is the part that confuses most people. In Rust, `pop()` returns an `Option<char>`, which is either:
- `Some(c)` when there was an item, or
- `None` when the stack was empty.

So we can't compare `pop()` to a plain `char`. We wrap our expected value as `Some(expected)` and compare like with like. If the stack was empty, we get `None != Some(...)`, which is true, so we return false. This is the Rust equivalent of the `undefined` trick in TypeScript.

```rust
stack.is_empty()
```
There is **no semicolon and no `return`**. In Rust, the last expression in a function is its return value. Adding a semicolon would turn it into a statement that returns nothing, which causes a compile error.

## Trace: `"([)]"` (should be `false`)

| Step | `ch` | Action | Stack after |
|---|---|---|---|
| 1 | `(` | opener, push | `[ ( ]` |
| 2 | `[` | opener, push | `[ (, [ ]` |
| 3 | `)` | closer, pop gives `[`, but `)` needs `(`. Mismatch, return `false` | |

The brackets are balanced in count, but the order is wrong, and the stack catches that.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Mutable variable | `let` (`const` for fixed) | `let mut` (`let` is fixed) |
| "Maybe a value" | `T \| undefined` | `Option<T>` |
| Pop from empty stack | `undefined` | `None` |
| Implicit return | no, needs `return` | yes, last expression with no `;` |
| Character loop | `for (const ch of s)` | `for ch in s.chars()` |