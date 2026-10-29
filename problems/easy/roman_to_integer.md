# Roman to Integer (deepseek generated)

## Problem

Given a Roman numeral string `s` (characters `I`, `V`, `X`, `L`, `C`, `D`, `M`), convert it to its integer value. In Roman numerals, a smaller symbol placed before a larger one is subtracted (e.g., `IV` = 4); otherwise values are added.

## Examples

| Input | Output | Why |
|---|---|---|
| `"III"` | `3` | 1 + 1 + 1 |
| `"LVIII"` | `58` | 50 + 5 + 1 + 1 + 1 |
| `"MCMXCIV"` | `1994` | 1000 + 900 + 90 + 4 |
| `"IV"` | `4` | 1 before 5 → subtract |
| `"IX"` | `9` | 1 before 10 → subtract |
| `"XII"` | `12` | 10 + 1 + 1 |

## Constraints

- `1 <= s.length <= 15`
- `s` contains only the characters `I`, `V`, `X`, `L`, `C`, `D`, `M`.
- It is guaranteed that `s` is a valid Roman numeral in the range [1, 3999].

## Things to think about before coding

- What makes `IV` different from `VI`? How do you detect a subtraction case?
- You only ever need to compare each symbol with the *next* one. Why is that enough?
- Can you do it in one pass with a small lookup table?

Give it a try before scrolling further.

---

## Solution

### Approach

Walk the string left to right. For each symbol, look at the next symbol: if the current value is smaller than the next, subtract it; otherwise add it. The last symbol is always added (compared to 0).

1. Build a lookup from symbol to value.
2. For each index `i`, get the current value and the next value (0 if there is no next).
3. If `current < next`, subtract `current`; else add `current`.
4. Return the running total.

**Time:** O(n). **Space:** O(1).

### TypeScript

```typescript
function romanToInt(s: string): number {
  const values: Record<string, number> = {
    I: 1,
    V: 5,
    X: 10,
    L: 50,
    C: 100,
    D: 500,
    M: 1000,
  };

  let total = 0;
  for (let i = 0; i < s.length; i++) {
    const curr = values[s[i]!]!;
    const next = i + 1 < s.length ? values[s[i + 1]!]! : 0;
    total += curr < next ? -curr : curr;
  }

  return total;
}

// Quick checks
console.log(romanToInt("III"));     // 3
console.log(romanToInt("LVIII"));   // 58
console.log(romanToInt("MCMXCIV")); // 1994
console.log(romanToInt("IV"));      // 4
```

### Rust

```rust
pub fn roman_to_int(s: &str) -> i32 {
    let mut total = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        let curr = value(c);
        let next = match chars.peek() {
            Some(&n) => value(n),
            None => 0,
        };

        if curr < next {
            total -= curr;
        } else {
            total += curr;
        }
    }

    total
}

fn value(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(roman_to_int("III"), 3);
        assert_eq!(roman_to_int("LVIII"), 58);
        assert_eq!(roman_to_int("MCMXCIV"), 1994);
        assert_eq!(roman_to_int("IV"), 4);
        assert_eq!(roman_to_int("IX"), 9);
    }
}
```

### Notes on the two implementations

- TypeScript uses a `Record<string, number>` lookup table plus a numeric index loop; Rust uses a `match`-based `value` helper and a peeking iterator.
- Both rely on the same "compare to the next symbol" rule.
- The Rust version uses `.peekable()` and `peek()` to look one character ahead without consuming it, which keeps space O(1) (no need to collect the string into a `Vec`).

## Likely interviewer follow-ups

1. **Int to Roman:** how would you do the reverse conversion (integer → Roman numeral)?
2. **Validation:** how would you extend this to detect *invalid* Roman numerals?
3. **Right-to-left scan:** could you also solve this by scanning from the right, and what would change?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Assign each symbol its value.
2. Walk left to right. For each symbol, look at the symbol after it.
3. If the current value is smaller than the next, subtract it (a "IV"-style pair). Otherwise add it.
4. The last symbol has no next, so it's always added.

## TypeScript, piece by piece

```typescript
const values: Record<string, number> = {
  I: 1, V: 5, X: 10, L: 50, C: 100, D: 500, M: 1000,
};
```
`Record<string, number>` means "an object whose keys are strings and whose values are numbers." It's a lookup table: `values["X"]` gives `10`.

```typescript
for (let i = 0; i < s.length; i++) {
```
A plain index loop, because we need to look at `i + 1` (the next character) too.

```typescript
const curr = values[s[i]!]!;
```
Two non-null assertions (`!`):
- `s[i]!` — under `noUncheckedIndexedAccess`, `s[i]` is `string | undefined`; we assert it's defined because `i < s.length`.
- `values[...]!` — indexing the `Record` also yields `number | undefined`; we assert it's defined because the input only contains valid Roman symbols.

```typescript
const next = i + 1 < s.length ? values[s[i + 1]!]! : 0;
```
The next symbol's value, or `0` when `i` is the last index (no next symbol). Using `0` makes the last symbol always "added" because any value is `>= 0`.

```typescript
total += curr < next ? -curr : curr;
```
The core rule in one line: if the current value is smaller than the next, add its negative (subtract); otherwise add it. `+= -curr` is subtraction.

## Rust, piece by piece

```rust
pub fn roman_to_int(s: &str) -> i32 {
```
`&str` is a borrowed string slice — the function looks at the string without taking ownership. `-> i32` is the return type.

```rust
let mut chars = s.chars().peekable();
```
`.chars()` yields the string's characters one at a time. `.peekable()` wraps the iterator so we can *peek* at the next item without consuming it. `mut` because iterating advances it.

```rust
while let Some(c) = chars.next() {
```
`chars.next()` returns `Option<char>`: `Some(c)` when there's a character left, `None` at the end. `while let` keeps looping while there's a `Some`, and binds `c` to the character.

```rust
let next = match chars.peek() {
    Some(&n) => value(n),
    None => 0,
};
```
`peek()` returns `Option<&char>` (a *reference* to the next char, or `None`). The pattern `Some(&n)` destructures the reference so `n` is a plain `char`. This is the Rust way to "look ahead without consuming."

```rust
fn value(c: char) -> i32 {
    match c {
        'I' => 1, 'V' => 5, 'X' => 10, 'L' => 50, 'C' => 100, 'D' => 500, 'M' => 1000,
        _ => 0,
    }
}
```
A helper that maps a character to its value. `match` is exhaustive; the `_ => 0` arm handles anything unexpected (it can't happen given the constraints). Single quotes (`'I'`) denote a `char`, not a string.

## Trace: `"MCMXCIV"` (should be `1994`)

| Step | `c` | `curr` | `next` | Action | `total` |
|---|---|---|---|---|---|
| 1 | M | 1000 | 100 | add | 1000 |
| 2 | C | 100 | 1000 | subtract | 900 |
| 3 | M | 1000 | 10 | add | 1900 |
| 4 | X | 10 | 100 | subtract | 1890 |
| 5 | C | 100 | 1 | add | 1990 |
| 6 | I | 1 | 5 | subtract | 1989 |
| 7 | V | 5 | 0 | add | 1994 |

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Lookup table | `Record<string, number>` object | `match`-based helper fn |
| "Maybe a value" | `T \| undefined` | `Option<T>` |
| Look ahead | `s[i + 1]` with bounds check | `iter.peek()` |
| Character access | `s[i]` (string index) | `s.chars().next()` |
| Destructure a reference | n/a | `Some(&n)` |
| Fallback for none | ternary `?: 0` | `None => 0` |
