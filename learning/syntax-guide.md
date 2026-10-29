# TypeScript and Rust Syntax, by Difficulty

This is a language-level companion to the [problem-solving guide](problem-solving-guide.md). That guide covers *how to think* about a problem; this one covers *what you must be able to type* to solve it at each difficulty tier.

Every entry is drawn from the reference solutions in [`solutions/`](../solutions/) and names the problems in the [catalog](../problems/README.md) that exercise it. Read the tier you are working on, attempt the problems it lists, and if a construct — not an algorithm — is what blocks you, fix the syntax first.

**Contents:** [Mental model](#1-the-mental-model) · [Easy](#2-easy) · [Medium](#3-medium) · [Hard](#4-hard) · [Failure modes](#5-cross-cutting-failure-modes) · [Drills](#6-practice-drills) · [References](#7-references)

## How to use this guide

- **Easy** is the floor: signatures, indexing, one pass, one hash map. If any Easy entry is unfamiliar, stop and drill it.
- **Medium** adds sorting with comparators, DP tables, grids, recursion with mutation, and the `entry` API.
- **Hard** adds designed state (classes and structs), index-based linking, monotonic structures, binary search invariants, and sentinels.
- Each tier **assumes the previous one**. Medium does not re-explain `Map`; Hard does not re-explain sorting.
- Snippets are **minimal fragments**, not complete programs. `// ...` marks elided code.
- "**Used in**" names come from the catalog. The reference solution for each is in `solutions/typescript/<tier>/` or `solutions/rust/<tier>/`.

### Two configuration facts that explain the code you will read

1. This repo's `tsconfig.json` sets **`noUncheckedIndexedAccess: true`**. Array and `Record` indexing therefore yields `T | undefined`, which is why the TypeScript solutions are full of `!` (`nums[i]!`). Without that flag, the same code compiles with no `!`; the flag is why you must decide what an out-of-range read means.
2. Rust snippets assume the usual imports are in scope:

```rust
use std::collections::{HashMap, HashSet, VecDeque};
```

---

## 1. The mental model

### 1.1 Two different contracts

**TypeScript** is JavaScript plus a checker. Types are erased before execution, so a type error never stops a running program, and `!` or `as` can silence the checker without changing behaviour. Values are references for objects and arrays; `undefined` is the "no value" marker. Mutability is a property of the *binding* (`const`), not of the object it points to.

**Rust** is a type system over ownership. A `Vec<T>` owns its heap buffer, `&[T]` borrows it immutably, and `&mut [T]` borrows it exclusively; the borrow checker rejects programs that alias a mutable borrow. Absence is `Option<T>`, not null. Every integer has a width, and the last expression of a block is its value.

Those two contracts produce most of the syntax differences below. When a Rust program will not compile, ask "who owns this, and who is borrowing it?"; when a TypeScript program type-checks but misbehaves, ask "what is `undefined` here, and who else holds this array?"

### 1.2 Master mapping table

| Intent | TypeScript | Rust |
|---|---|---|
| Declare a function | `function f(x: number): number` | `fn f(x: i32) -> i32` |
| Return a value | `return x;` | `x` as the last expression, no `;` |
| No return value | `: void` | omit the `->` |
| A value that may be missing | `T \| undefined`, `T \| null` | `Option<T>` (`Some`/`None`) |
| Handle a missing value | `if (x !== undefined)` | `if let Some(x) = ...` / `match` |
| Mutable local | `let x = 0; x += 1;` | `let mut x = 0; x += 1;` |
| Mutate a caller's array | pass the array (reference semantics) | `&mut Vec<T>` or `&mut [T]` |
| Read-only borrow | (types only) `readonly T[]` | `&[T]` |
| Index type | `number` (double, signed) | `usize` (unsigned) |
| Index may be out of range | element is `T \| undefined` under this config | panic, or `.get(i) -> Option<&T>` |
| Growable array | `T[]` / `Array<T>` | `Vec<T>` |
| Fixed-size array | `T[]` (length not in the array type; tuples `[A, B]` fix it) | `[T; N]` |
| Keyed collection | `Map<K, V>` | `HashMap<K, V>` (or `BTreeMap`) |
| Membership set | `Set<T>` | `HashSet<T>` |
| Sort numbers | `arr.sort((a, b) => a - b)` | `arr.sort_unstable()` |
| Iterate with an index | `for (const [i, x] of arr.entries())` | `for (i, &x) in arr.iter().enumerate()` |
| Lookup that may miss | `map.get(k)` → `V \| undefined` | `map.get(&k)` → `Option<&V>` |
| Insert-or-update | `map.set(k, (map.get(k) ?? 0) + 1)` | `*map.entry(k).or_insert(0) += 1` |
| String as a parameter | `s: string` | `s: &str` |
| Owned string | `string` (always) | `String` |
| Character access | `s[i]`, `s.charCodeAt(i)` | `s.as_bytes()[i]`, `s.chars().nth(i)` |
| Build a string | `parts.join("")` | `chars.collect::<String>()` |
| Integer division | `Math.floor(a / b)` or `Math.trunc(a / b)` | `a / b` (truncates) |
| Overflow | silently loses precision above 2⁵³ | panic in debug, wraps in release |
| "Impossible" branch | `throw new Error("...")` | `unreachable!("...")` |
| Tests | `console.log` checks | `#[cfg(test)] mod tests` + `assert_eq!` |

### 1.3 What the tiers contain

| Tier | Problems | Syntax burden |
|---|---|---|
| Easy | 13 | one pass or one pointer pair; one `Map`/`Set`; signatures; indexing; char codes; in-place swap |
| Medium | 12 | comparators; DP arrays; grids and recursion; `entry` API; slices; intervals; dedup |
| Hard | 8 | classes/structs; `Option`-linked nodes; `VecDeque`; binary search; backtracking with undo; sentinels |

---

## 2. Easy

Easy problems need no data-structure design. The syntax you need is: declare a typed function, walk an array or string, keep one accumulator, and put one lookup table beside it.

### 2.1 TypeScript: the Easy toolkit

#### Function signatures and return types

```typescript
function twoSum(nums: number[], target: number): number[] {
  // ...
  return [];
}

const xorAll = (nums: number[]): number => nums.reduce((a, b) => a ^ b, 0);
```

Annotations are required by the repo's `strict` mode when a parameter's type is not inferable. `number[]` and `Array<number>` are the same type; `void` marks a function called for effect.

**Gotcha.** `number` covers integers and floats. There is no `int`; `/` always produces a float, so integer division is `Math.trunc(a / b)` (or `Math.floor` when both operands are non-negative).

**Used in:** every Easy problem.

#### `const` and `let`

```typescript
const n = nums.length; // binding fixed
let best = 0;          // binding reassigned
```

`const` fixes the *binding*, not the value: `const arr = []; arr.push(1)` is legal, and `const` parameters are routinely mutated in place (`rotate_array`, `plus_one`).

**Used in:** all.

#### Indexing, `noUncheckedIndexedAccess`, and `!`

```typescript
const first = nums[0]!;        // asserted: caller guarantees non-empty
const last = nums[nums.length - 1]!;
```

With this repo's config, `nums[0]` has type `number | undefined`, because the checker cannot know the array is non-empty. `!` removes `undefined` and is erased at runtime — it does not add a check. Prefer a guard (`if (i >= arr.length) return ...`) at real boundaries, and reserve `!` for indices you have just proven in range.

**Gotcha.** `!` on an actually-out-of-range read still yields `undefined`, and the next arithmetic operation produces `NaN` silently. Rust would have panicked at the same point; here the failure is deferred.

**Used in:** virtually every solution; `plus_one`, `rotate_array`, `median_of_two_sorted_arrays`.

#### `for...of`, classic `for`, and early exit

```typescript
for (const ch of s) { /* ... */ }                    // values
for (const [i, num] of nums.entries()) { /* ... */ } // index + value
for (let i = s.length - 1; i >= 0; i--) { /* ... */ } // backwards

for (const num of nums) {
  if (num < 0) break; // `forEach` cannot break; `for...of` can
}
```

**Gotcha.** `for...in` iterates *string keys*, not values — never use it on arrays. Use `for...of` for values, `.entries()` for index/value pairs.

**Used in:** `valid_palindrome`, `plus_one`, `roman_to_integer`, `longest_valid_parentheses`.

#### `Map`: the default lookup table

```typescript
const seen = new Map<number, number>(); // value -> index
const found = seen.get(complement);     // number | undefined
if (found !== undefined) return [found, i];
seen.set(num, i);
```

`get` returns `V | undefined`; `has`, `delete`, and `size` round it out. `Map` preserves insertion order, which `group_anagrams` and `top_k_frequent_elements` rely on for deterministic output.

**Gotcha.** `map.get(k) ?? 0` is the counting idiom; `map.get(k) || 0` is *not* equivalent when stored counts can be `0`.

**Used in:** `two_sum`, `group_anagrams`, `top_k_frequent_elements`, `minimum_window_substring`, `lru_cache`, `longest_substring_without_repeating_characters`.

#### `Set`: membership in O(1)

```typescript
const seen = new Set<number>();
if (seen.has(x)) return true;
seen.add(x);
```

`[...set]` materialises a `Set` as an array; `set.size` is the count. `contains_duplicate` is a three-line `Set` program.

**Used in:** `contains_duplicate`, `n_queens`.

#### `Record` and the `in` guard

```typescript
const closeToOpen: Record<string, string> = { ")": "(", "]": "[", "}": "{" };
if (ch in closeToOpen) { /* look up closeToOpen[ch] */ }
```

A `Record<K, V>` is a plain object typed as a dictionary. Under `noUncheckedIndexedAccess`, `closeToOpen[ch]` is `V | undefined`, so `in` (or a prior `!== undefined`) is what narrows it.

**Used in:** `valid_brackets`, `roman_to_integer`.

#### Strings: indexing, `charCodeAt`, `split`/`join`

```typescript
const ch = s.charAt(i);            // string ("" out of range)
const code = s.charCodeAt(i);      // number, UTF-16 code unit
const letters = str.split("").sort().join(""); // anagram key
const lower = c.toLowerCase();
```

`s[i]` and `s.charAt(i)` differ at the boundary: `s[i]` is `string | undefined`, `charAt` returns `""`. UTF-16 means `.length` counts code units, not code points — fine for the ASCII constraints in this repo.

**Used in:** `valid_palindrome`, `valid_anagram`, `group_anagrams`, `first_unique_character`.

#### Numbers: `Math`, `%`, and `Infinity`

```typescript
const mid = Math.floor((low + high) / 2);
const abs = Math.abs(x);
const best = Math.min(best, candidate);
let maxProfit = 0, minPrice = Infinity;
```

`%` keeps the sign of the dividend (`-7 % 3 === -1`), unlike Python's modulo. `Infinity` is a real `number` and compares greater than everything, which makes it a natural "no answer yet" sentinel (`coin_change`, `median_of_two_sorted_arrays`).

**Used in:** `median_of_two_sorted_arrays`, `coin_change`, `best_time_to_buy_and_sell_stock`, `container_with_most_water`, `trapping_rain_water`.

#### In-place mutation, swapping, and copies

```typescript
const tmp = nums[i]!;
nums[i] = nums[j]!;
nums[j] = tmp;
// or, for two variables:
[a, b] = [b, a];
```

Arrays are passed by reference: mutating a parameter mutates the caller's array, which is exactly what `rotate_array`, `reverse_string`, and `remove_duplicates_from_sorted_array` require. When you need a copy, say so: `[...arr]` or `arr.slice()`.

**Used in:** `reverse_string`, `rotate_array`, `remove_duplicates_from_sorted_array`, `merge_intervals`.

#### Bitwise XOR

```typescript
let result = 0;
for (const n of nums) result ^= n;
```

Bitwise operators coerce to **32-bit signed integers**. That is safe for this repo's `i32` bounds and the reason `single_number` works; it would silently truncate larger values.

**Used in:** `single_number`.

### 2.2 Rust: the Easy toolkit

#### `fn` signatures and the last expression

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    // ...
    vec![]
}
```

No `return` is needed: the final expression, without a semicolon, is the value. `return` still works for early exits. The `pub` makes the function reachable from the test module.

**Gotcha.** Adding a semicolon to the last expression turns the function's value into `()`, producing a type error. Semicolons are load-bearing in Rust in a way they are not in TypeScript.

**Used in:** every Easy solution.

#### `let`, `let mut`, and shadowing

```rust
let n = nums.len();
let mut best = 0;
best = best.max(x);

let n = n as i32; // shadowing: a new binding of a new type
```

Bindings are immutable unless declared `mut`; the compiler rejects assignment to a non-`mut` binding. Shadowing (a second `let` with the same name) is idiomatic for type changes (`usize` → `i32`).

**Used in:** all.

#### Scalar types, `usize`, and `as`

```rust
let count = 0i32;
let index: usize = 0;       // index and length type
let n = nums.len() as i32;  // explicit cast
let byte = b'a';            // u8 literal
```

`i32` is the default integer; `usize` is what indexes and lengths use. `as` is a **truncating, unchecked** cast: `300i32 as u8` is `44`, and `-1i32 as usize` is `usize::MAX`. Cast in both directions at API boundaries, as the repo's signatures force (`Vec<i32>` in, `usize` for indexing).

**Gotcha.** You cannot mix integer types in arithmetic: `nums.len() - 1` is `usize`, `i` may be `i32`, and `nums[i]` will not compile until you cast.

**Used in:** `two_sum`, `first_unique_character`, `longest_palindromic_substring`, `median_of_two_sorted_arrays`.

#### `Vec<T>`: creation, indexing, and growth

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
v.push(4);
let first = v[0];             // panics if empty
let maybe = v.get(0);         // Option<&i32>
let n = v.len();
let empty = v.is_empty();
```

`vec!` is the literal macro; nested `vec![vec![0; cols]; rows]` builds a grid (used at Medium). Indexing panics on out-of-range; `.get` returns `Option` when a miss is normal.

**Gotcha.** `v.len() - 1` underflows (and panics in debug builds) when `v` is empty. Guard with `is_empty()` first, as `valid_palindrome` does before computing `bytes.len() - 1`.

**Used in:** all; `valid_palindrome` for the empty guard.

#### Iteration: `iter()`, `.enumerate()`, and pattern `&`

```rust
for num in &nums { /* num: &i32 */ }
for (i, &num) in nums.iter().enumerate() { /* num: i32 */ }
for &b in s.as_bytes() { /* b: u8 */ }
```

Iterating `&Vec<T>` yields references. The `&num` pattern *destructures* the reference so arithmetic works on the value. `for x in nums` (no `&`) moves the vector and ends its life.

**Gotcha.** `for num in &nums` then `num + 1` fails: you have `&i32`, not `i32`. Add `&` in the pattern or dereference (`*num`).

**Used in:** `two_sum`, `single_number`, `valid_anagram`, `valid_palindrome`.

#### `if`/`else` as expressions; `match`

```rust
let sign = if x < 0 { -1 } else { 1 };

match c {
    'I' => 1,
    'V' => 5,
    _ => 0,
}
```

`match` must be exhaustive; `_` covers the rest. Arms are expressions, so a `match` can initialise a variable directly — the basis of `value(c)` in `roman_to_integer`.

**Used in:** `roman_to_integer`, `valid_brackets`.

#### `Option<T>`: the absence type

```rust
let found: Option<usize> = seen.get(&complement).copied();
if let Some(j) = found {
    return vec![j as i32, i as i32];
}
let count = counts.get(&c).copied().unwrap_or(0);
```

There is no `null`. `HashMap::get` returns `Option<&V>`; `.copied()` turns that into `Option<V>` for `Copy` types. `if let` handles the `Some` case and ignores `None`; `match` handles both explicitly.

**Gotcha.** `if let` silently drops the `None` branch. When "missing" needs an action, use `match` or `else` (`if let ... { } else { }`), as `roman_to_integer` does when peeking at the next character.

**Used in:** `two_sum`, `roman_to_integer`, `longest_substring_without_repeating_characters`.

#### `&str` versus `String`

```rust
pub fn is_palindrome(s: &str) -> bool { /* ... */ }   // borrows any string
pub fn two_sum(...) -> Vec<i32> { /* ... */ }
let owned: String = "abc".to_string();
let built = String::new();
```

Take `&str` in parameters (accepts `&String` and literals). Return `String` when you build new text. `chars()` iterates `char` (Unicode scalar values), `bytes()` and `as_bytes()` iterate `u8`.

**Gotcha.** `&str` is **not indexable by character**: `s[0]` does not compile. For ASCII, `s.as_bytes()[0]` works (valid in `valid_palindrome` because the constraints are ASCII); for general text, `s.chars().nth(0)` is O(n).

**Used in:** `valid_palindrome`, `valid_anagram`, `roman_to_integer`, `first_unique_character`.

#### `println!`, `{:?}`, and tests

```rust
println!("{:?}", two_sum(vec![2, 7, 11, 15], 9)); // [0, 1]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(two_sum(vec![2, 7], 9), vec![0, 1]);
    }
}
```

`{}` needs `Display`; `{:?}` needs `Debug`. The test module is compiled only for `cargo test`, and `use super::*` imports the parent's items.

**Used in:** every Rust solution.

### 2.3 Easy: cross-language notes

- A TS `number[]` parameter is a *reference*; a Rust `Vec<i32>` parameter *takes ownership*. Rust in-place solutions therefore declare `&mut Vec<T>` (`reverse_string`, `rotate_array`).
- TS signals "not found" with `undefined`; Rust with `None`. The check is `!== undefined` versus `if let Some(...)`.
- Rust has no implicit numeric conversion; TS has no integer type. Both force you to be explicit at boundaries — Rust with `as`, TS with `Math.floor`/`Math.trunc`.
- A TS `!` and a Rust `unwrap()` are both assertions that can fail at runtime. Rust's fails loudly and immediately; TS's produces `undefined`/`NaN` downstream.

### 2.4 Easy readiness checklist

- [ ] Write a function signature in both languages from memory, including an array parameter and a return type.
- [ ] Iterate an array with and without indices, forwards and backwards.
- [ ] Count occurrences with `Map` / `HashMap`, including the "missing key" case.
- [ ] Swap two elements in place; explain why TS mutates the caller's array and Rust needs `&mut`.
- [ ] Explain why `nums[i]` is `number | undefined` here, and when `!` is honest.
- [ ] Explain why `bytes.len() - 1` needs an empty check in Rust.
- [ ] Convert a `char`/byte to an index (`b'a'`, `charCodeAt`) and back.
- [ ] Run one worksheet and its solution in both languages, and run `cargo test` on the solution.

---

## 3. Medium

Medium problems combine two structures or add a second dimension: a sorted array with two pointers, a DP table indexed by amount, a grid walked recursively, a frequency map turned into buckets. The syntax burden is *composition* — several features in one function, each of which must be right.

### 3.1 TypeScript: the Medium toolkit

#### Sorting with a comparator

```typescript
const sorted = [...intervals].sort((a, b) => a[0]! - b[0]!); // copy, then sort
nums.sort((a, b) => a - b);                                  // in place
```

Default `sort()` compares **strings**: `[10, 9, 1].sort()` is `[1, 10, 9]`. Always pass `(a, b) => a - b` for numbers and `(a, b) => keyOf(a) - keyOf(b)` for tuples. The comparator returns negative/zero/positive.

**Gotcha.** `.sort` mutates in place. Copy first (`[...arr]`) when the caller's order matters, and remember that mutating the *rows* of the copy still mutates the original intervals.

**Used in:** `merge_intervals`, `three_sum`, `group_anagrams`.

#### `Array.from`, `new Array`, and `.fill`

```typescript
const dp = new Array<number>(amount + 1).fill(Infinity);
const buckets: number[][] = Array.from({ length: n + 1 }, () => []);
const board = Array.from({ length: n }, () => new Array<string>(n).fill("."));
```

`Array.from({ length: n }, fn)` calls `fn` per index — the safe way to build nested structures, because each call returns a *fresh* inner array.

**Gotcha.** `new Array(n).fill([])` fills every slot with the **same** array reference; writing to one row writes to all. Use `Array.from({ length: n }, () => [])` instead. Also note `new Array(n)` creates holes, which `noUncheckedIndexedAccess` types as `T | undefined`.

**Used in:** `coin_change`, `top_k_frequent_elements`, `n_queens`, `sliding_window_maximum`, `product_of_array_except_self`.

#### Iterating and materialising a `Map`

```typescript
for (const [key, value] of freq) { /* ... */ }
const values = Array.from(groups.values());
const keys = [...freq.keys()];
```

`Map` is iterable as `[key, value]` pairs, and preserves insertion order. `Array.from(map.values())` is the standard way to return a `Map`'s contents as an array (`group_anagrams`).

**Used in:** `group_anagrams`, `top_k_frequent_elements`, `minimum_window_substring`.

#### `??`, `?.`, and defaults

```typescript
const count = freq.get(x) ?? 0;
const first = arr[0] ?? 0;
function f(nums: number[], k = 1): number { /* ... */ }
```

`??` falls back only on `null`/`undefined`; `||` also falls back on `0`, `""`, and `false`, which corrupts counters. Optional parameters (`k?: number`) and defaults (`k = 1`) are how the repo's signatures stay compact.

**Gotcha.** `??` chains left-to-right; `a ?? b ?? 0` is fine, but mixing `??` with `||`/`&&` without parentheses is a syntax error.

**Used in:** `top_k_frequent_elements`, `minimum_window_substring`, `merge_k_sorted_lists`.

#### DP arrays and sentinels

```typescript
const dp = new Array<number>(amount + 1).fill(Infinity);
dp[0] = 0;
for (let a = 1; a <= amount; a++) {
  for (const coin of coins) {
    if (coin <= a) dp[a] = Math.min(dp[a]!, dp[a - coin]! + 1);
  }
}
```

Index the table by the *subproblem* (`a` = amount), initialise unreachable states to `Infinity`, and let `Math.min` do the work. The final answer converts the sentinel back to `-1`.

**Gotcha.** `Infinity + 1` is `Infinity`, so an unreachable predecessor never produces a bogus finite count — that is precisely why `Infinity` is the right sentinel, and why `0` or `-1` would be wrong.

**Used in:** `coin_change`, `climbing_stairs`, `maximum_subarray`.

#### Grids, nested loops, and recursive flood fill

```typescript
const rows = grid.length, cols = grid[0]!.length;

function dfs(r: number, c: number): void {
  if (r < 0 || c < 0 || r >= rows || c >= cols || grid[r]![c] !== "1") return;
  grid[r]![c] = "0"; // mark visited by overwriting
  dfs(r + 1, c); dfs(r - 1, c); dfs(r, c + 1); dfs(r, c - 1);
}
```

The nested function closes over `grid`, `rows`, and `cols`, mutating the grid in place. Bounds checks come first; marking before recursing is what prevents infinite cycles.

**Gotcha.** The recursion depth is `rows × cols` in the worst case (one long snake); it is fine for this repo's constraints but would overflow the stack for a million-cell grid.

**Used in:** `number_of_islands`.

#### Intervals and the "compare with last" loop

```typescript
const merged: number[][] = [];
for (const interval of sorted) {
  const last = merged[merged.length - 1];
  if (last === undefined || interval[0]! > last[1]!) merged.push(interval);
  else last[1] = Math.max(last[1]!, interval[1]!);
}
```

`last === undefined` is the empty-result case; the tuple comparison decides overlap. Writing `last[1]` mutates the interval object, not a copy.

**Gotcha.** A `number[][]` row is a reference; if the caller's intervals must not change, clone each row (`[...interval]`).

**Used in:** `merge_intervals`.

#### Frequency buckets

```typescript
const buckets: number[][] = Array.from({ length: nums.length + 1 }, () => []);
for (const [num, count] of freq) buckets[count]!.push(num);
```

The count *is* the index, giving O(n) selection instead of O(n log n) sorting. Iterate the bucket array backwards to get the most frequent first.

**Used in:** `top_k_frequent_elements`.

### 3.2 Rust: the Medium toolkit

#### The `entry` API

```rust
let mut freq: HashMap<i32, usize> = HashMap::new();
*freq.entry(num).or_insert(0) += 1;      // count
groups.entry(key).or_default().push(s);   // map -> Vec
```

`entry` looks the key up once and gives a handle to the slot: `or_insert(default)` yields `&mut V`, `or_default()` uses `V::default()`. This is the Rust idiom for "get or create, then mutate" and replaces the TS `get ?? 0` dance.

**Gotcha.** The `or_insert` argument is evaluated eagerly. For an expensive default, use `or_insert_with(|| ...)`.

**Used in:** `group_anagrams`, `top_k_frequent_elements`, `minimum_window_substring`.

#### Sorting and deduplication

```rust
intervals.sort_by_key(|v| v[0]);        // stable, by key
nums.sort_unstable();                    // fast, order of equals unspecified
nums.dedup();                            // removes *consecutive* duplicates
if let Some(last) = merged.last_mut() { last[1] = last[1].max(v[1]); }
```

`sort_unstable` avoids allocation; `sort_by` takes a full comparator (`|a, b| b.cmp(a)` for descending). `dedup` only collapses adjacent equal values, so it must follow a sort.

**Used in:** `merge_intervals`, `three_sum`, `group_anagrams`.

#### Mutable parameters and in-place algorithms

```rust
pub fn rotate(nums: &mut Vec<i32>, k: i32) { /* mutate */ }
pub fn reverse_string(s: &mut Vec<char>) { s.reverse(); }
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 { /* ... */ }
```

`&mut` is an exclusive borrow: the caller must pass `&mut v`, and no other reference to `v` may be live. In-place problems therefore take `&mut Vec<T>` (or `&mut [T]`) and may return a count (`remove_duplicates`).

**Gotcha.** You cannot hold `let first = &nums[0];` and then call a `&mut nums` method; the immutable borrow is still live. Scope borrows to end them (`let first = nums[0];` copies for `Copy` types).

**Used in:** `reverse_string`, `rotate_array`, `remove_duplicates_from_sorted_array`.

#### Slices and ranges

```rust
fn expand(bytes: &[u8], left: i32, right: i32) -> (i32, i32) { /* ... */ }
let head = &nums[..k];
let tail = &nums[k..];
for i in 0..n { /* 0..n excludes n */ }
for i in 0..=n { /* includes n */ }
```

`&[T]` is a borrowed view into a `Vec<T>` or array and is the idiomatic read-only parameter. Index ranges are half-open; `..=` is inclusive. Slicing copies nothing.

Some solutions keep an **exclusive upper bound** (`let mut right = height.len();` then index `right - 1`) instead of `len - 1`; that avoids the empty-input underflow entirely (`trapping_rain_water`).

**Gotcha.** `&s[a..b]` on a `&str` slices **bytes** and panics if `a` or `b` is not a character boundary — safe only under the repo's ASCII assumptions.

**Used in:** `longest_palindromic_substring`, `remove_duplicates_from_sorted_array`, `minimum_window_substring`, `trapping_rain_water`.

#### Recursion with `&mut` state

```rust
fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
    if r >= rows || c >= cols || grid[r][c] != '1' { return; }
    grid[r][c] = '0';
    dfs(grid, r + 1, c, rows, cols);
    // ...
}
dfs(&mut grid, r, c, rows, cols);
```

A nested `fn` **cannot capture** the enclosing variables, so every piece of state is an explicit parameter. `&mut` is re-borrowed on each recursive call; the compiler tracks the exclusive borrow for the duration of the call, which is exactly the guarantee the algorithm needs.

**Used in:** `number_of_islands`, `n_queens`.

#### Numeric guards: `saturating`, `checked`, `MIN`/`MAX`

```rust
for i in 0..n.saturating_sub(2) { /* n < 2 => empty range */ }
let safe = a.checked_sub(b);            // Option<i32>
let best = i32::MAX;
let impossible = usize::MAX;
```

`saturating_sub` clamps at zero instead of underflowing — the standard defense against `usize` subtraction. `checked_*` returns `Option`, `i32::MIN`/`MAX` are sentinels for "no bound yet", and `usize::MAX` is the unsigned equivalent.

**Used in:** `three_sum`, `median_of_two_sorted_arrays`, `minimum_window_substring`.

#### String building and byte-level work

```rust
let key: String = chars.into_iter().collect();     // Vec<char> -> String
let s = s[start as usize..end as usize].to_string(); // slice -> owned
let lower = b.to_ascii_lowercase();                 // u8 method
```

`collect::<String>()` is the canonical build-from-parts call; `.to_string()` copies a borrowed slice into an owned `String`. Byte methods (`to_ascii_lowercase`, `is_ascii_alphanumeric`) exist on `u8`, not on `char`, which is why the byte-level versions appear in ASCII problems.

**Used in:** `group_anagrams`, `longest_palindromic_substring`, `valid_palindrome`.

#### Iterator adapters: `zip`, `any`, `all`, `map`/`collect`

```rust
for (sc, tc) in s.bytes().zip(t.bytes()) { /* pairs */ }
counts.iter().all(|&c| c == 0)
let total: i32 = nums.iter().sum();
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
```

Adapters are lazy until a consumer runs (`collect`, `sum`, `all`, `for`). A closure's parameter pattern can destructure (`|&c|`).

**Used in:** `valid_anagram`, `group_anagrams`.

#### `while let`, `peekable`, and option chaining

```rust
let mut chars = s.chars().peekable();
while let Some(c) = chars.next() {
    let next = match chars.peek() {
        Some(&n) => value(n),
        None => 0,
    };
    // ...
}
```

`peekable()` adds `peek()` without consuming; `while let Some(x) = ...` is the loop form of `if let`. `match` on `peek()` gives the lookahead its default when the string ends.

**Used in:** `roman_to_integer`.

#### `Vec<Vec<T>>` grids

```rust
let mut grid: Vec<Vec<char>> = vec![vec!['1'; cols]; rows];
grid[r][c] = '0';
```

Nested indexing is `grid[r][c]`, not `grid[r, c]`. Building with `vec![vec![...; cols]; rows]` clones the inner row; for non-`Copy` content, build rows with `(0..rows).map(|_| ...).collect()`.

**Used in:** `number_of_islands`, `n_queens`.

#### Cloning versus moving

```rust
let copy = nums.clone();
let key = s.to_string();          // &str -> String
fn takes(v: Vec<i32>) { /* owns v */ }
takes(nums);                      // nums moved; unusable afterwards
```

Passing a `Vec` or `String` by value moves it. `.clone()` copies explicitly; `.to_string()` produces an owned string from a borrow. The compiler error "use of moved value" points at the line where ownership left.

**Used in:** `group_anagrams`, `merge_intervals`, `n_queens`.

### 3.3 Medium: cross-language notes

- TS sorting is a comparator call you pass; Rust splits it into `sort_unstable` (natural order), `sort_by_key`, and `sort_by` (custom). Both mutate in place; TS needs an explicit copy.
- TS builds nested arrays with `Array.from` (fresh inner arrays); Rust builds them with `vec![vec![...]]` (cloned inner rows) — both have a shared-reference trap, in different forms.
- The `entry` API has no direct TS equivalent; the closest is `map.get(k) ?? 0` followed by `set`. In Rust, `entry` avoids the double lookup.
- TS DP tables are `number[]` initialised to `Infinity`; Rust tables are `Vec<i32>` (or `Vec<usize>`) initialised to a sentinel such as `usize::MAX`, or `Option<i32>` when "unreachable" must be distinct from a large number.
- TS grid DFS closes over the grid; Rust passes `&mut grid` explicitly because nested functions cannot capture.

### 3.4 Medium readiness checklist

- [ ] Sort numbers, tuples, and strings correctly, and explain why the default TS sort is wrong for numbers.
- [ ] Build a 2-D array/`Vec<Vec<_>>` without sharing inner arrays/rows by accident.
- [ ] Write a DP recurrence as a loop over the table indices, with an unreachable sentinel.
- [ ] Write a recursive flood fill in both languages and mark visited correctly.
- [ ] Merge intervals from a sorted list, including the empty case.
- [ ] Count with `entry(...).or_insert(0)` in Rust and `get(k) ?? 0` in TS.
- [ ] Explain the difference between `&[T]`, `&mut Vec<T>`, and `Vec<T>` parameters, and pick the right one.
- [ ] Handle `usize` subtraction without underflow.

---

## 4. Hard

Hard problems are won or lost on *state design*: a cache whose links must be relinked in O(1), a deque that must stay monotonic, a binary search whose invariant must be stated, a backtracking search that must undo exactly what it changed. The syntax at this level is the syntax of *designed types* and *precise invariants*.

### 4.1 TypeScript: the Hard toolkit

#### Classes, private fields, and methods

```typescript
class LRUCache {
  private capacity: number;
  private map: Map<number, number>;

  constructor(capacity: number) {
    this.capacity = capacity;
    this.map = new Map();
  }

  get(key: number): number {
    // ...
    return -1;
  }
}

const cache = new LRUCache(2);
```

`private` is a compile-time restriction only; it does not hide the field at runtime. `this` is bound per call, so a method passed as a callback loses it — call methods as `this.method()` or bind them.

**Gotcha.** A nested `function` inside a method does **not** inherit `this`; use an arrow function or capture `const self = this`. This is the classic cache-implementation bug.

**Used in:** `lru_cache`.

#### Interfaces and structural typing

```typescript
interface LRUNode {
  key: number;
  value: number;
  prev: number; // index into nodes, -1 = none
  next: number;
}

const nodes: LRUNode[] = [];
nodes.push({ key, value, prev: -1, next: -1 });
```

An `interface` names an object shape; any object with those fields is assignable. There is no constructor and no runtime presence. Optional fields use `?` and, under `exactOptionalPropertyTypes`, `undefined` is not the same as absent.

**Used in:** `lru_cache`.

#### Non-null assertion in proven-hot paths

```typescript
const idx = this.map.get(key);
if (idx === undefined) return -1;
const value = this.nodes[idx]!.value; // idx proven in range by the map's invariant
```

The `!` here encodes an invariant the checker cannot see: every index stored in `map` is a valid index into `nodes`. That is acceptable *because the class maintains it on every mutation*. If you cannot state the invariant in one sentence, replace `!` with a check.

**Gotcha.** `this.nodes[idx]!` on a hole or an out-of-range index yields `undefined`, and `.value` then throws `TypeError` — at the exact line, at least, but only when that path runs.

**Used in:** `lru_cache`.

#### Sentinels: `Infinity` and `Number.MAX_SAFE_INTEGER`

```typescript
let bestLen = Infinity;
const left1 = i === 0 ? -Infinity : nums1[i - 1]!;
return bestLen === Infinity ? "" : s.slice(bestLeft, bestLeft + bestLen);
```

`Infinity` beats every finite number, so it works as "no candidate yet"; `-Infinity` works as "no left neighbour". Because `number` is a double, comparisons stay total and no overflow can occur.

**Gotcha.** `Infinity - Infinity` is `NaN`, and `NaN` fails every comparison — sentinel arithmetic must avoid subtracting two infinities.

**Used in:** `median_of_two_sorted_arrays`, `minimum_window_substring`, `coin_change`.

#### Building and rendering grids

```typescript
const board = Array.from({ length: n }, () => new Array<string>(n).fill("."));
results.push(board.map((row) => row.join("")));
```

`map` + `join("")` converts a `string[][]` board into the `string[]` representation LeetCode expects. `Array.from` guarantees an independent row per index.

**Used in:** `n_queens`.

#### Backtracking: choose, recurse, un-choose

```typescript
const cols = new Set<number>();
function backtrack(row: number): void {
  if (row === n) { results.push(render()); return; }
  for (let col = 0; col < n; col++) {
    if (cols.has(col)) continue;
    cols.add(col);
    board[row]![col] = "Q";
    backtrack(row + 1);
    board[row]![col] = ".";   // undo, exactly once
    cols.delete(col);
  }
}
```

The undo block must mirror the choose block exactly. `Set` makes the conflict test O(1), and the recursion depth is `n`.

**Gotcha.** Forgetting one undo leaves stale state that corrupts *later* branches — the failure looks like a wrong answer, not a crash. Write the choose and un-choose lines adjacent, as above.

**Used in:** `n_queens`.

#### Binary search on a partition

```typescript
let low = 0, high = m;
while (low <= high) {
  const i = Math.floor((low + high) / 2);
  const j = half - i;
  if (left1 <= right2 && left2 <= right1) return /* found */;
  else if (left1 > right2) high = i - 1;
  else low = i + 1;
}
throw new Error("input arrays are not sorted");
```

State the invariant before writing the loop ("the partition is correct when every left element ≤ every right element"). `Math.floor` matters: `(low + high) / 2` is a float, and using it as an index reads `nums[1.5]` → `undefined`. The trailing `throw` documents that the loop is unreachable for valid input; it is the TS analogue of Rust's `unreachable!`.

**Used in:** `median_of_two_sorted_arrays`.

#### The stack-of-indices pattern

```typescript
const stack: number[] = [-1]; // sentinel base index
for (let i = 0; i < s.length; i++) {
  if (s[i] === "(") stack.push(i);
  else {
    stack.pop();
    if (stack.length === 0) stack.push(i);
    else maxLen = Math.max(maxLen, i - stack[stack.length - 1]!);
  }
}
```

Storing *indices* rather than characters lets the length be computed as a difference; the `-1` base handles a match that starts at index 0.

**Used in:** `longest_valid_parentheses`.

### 4.2 Rust: the Hard toolkit

#### `struct` + `impl` + `#[derive]`

```rust
#[derive(Clone, Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node { key, value, prev: None, next: None }
    }
}
```

`impl` holds the methods; `Self` is the type being implemented. Fields are private to the module by default, so `pub struct` with private fields is the normal encapsulation pattern. `#[derive(Debug)]` is required before `println!("{:?}", node)`.

**Gotcha.** `#[derive(Clone)]` on a struct containing `Vec` clones the whole vector. Derive `Copy` only for small plain-data types; a `Copy` struct cannot contain a `String` or `Vec`.

**Used in:** `lru_cache`, `merge_k_sorted_lists`.

#### `Option<usize>` for links, and `.copied()`

```rust
match self.map.get(&key).copied() {
    Some(idx) => { /* idx: usize */ }
    None => -1,
}
```

A linked structure stores indices, and "no link" is `None` rather than `-1`/`usize::MAX`. `.copied()` converts `Option<&usize>` to `Option<usize>` (only for `Copy` types), which is what lets the match bind `usize` by value.

**Gotcha.** `Option<usize>` costs an extra word in memory versus a sentinel `usize::MAX`, but it makes the invalid state unrepresentable. Prefer it unless the sentinel is genuinely simpler (see `minimum_window_substring`).

**Used in:** `lru_cache`, `merge_k_sorted_lists`.

#### `Box` for recursive types; tail pointers with `&mut Option<Box<T>>`

```rust
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn merge_two_lists(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>)
    -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;
    loop {
        match (a, b) {
            (Some(x), Some(y)) => {
                let mut x = x;
                let rest = x.next.take(); // detach, leaving None behind
                *tail = Some(x);
                a = rest;
                // ...
            }
            (None, rest) | (rest, None) => {
                *tail = rest;
                break;
            }
        }
    }
    head
}
```

`Box` gives a recursive type a fixed size: `Option<Box<ListNode>>` is one pointer wide. `&mut Option<Box<ListNode>>` is a "tail pointer" into the list being built; assigning through it (`*tail = ...`) appends. `.take()` moves the `next` field out while leaving `None`, which the borrow checker accepts. The or-pattern `(None, rest) | (rest, None)` handles "one side exhausted".

**Gotcha.** `tail = &mut node.next` re-borrows the node you just moved; you must reassign `tail` through a `match` after each append, exactly as above. Without `take()`, `x.next` cannot be moved out of `x` while `x` is owned by the match arm.

**Used in:** `merge_k_sorted_lists`.

#### `VecDeque`: the monotonic queue

```rust
let mut deque: VecDeque<usize> = VecDeque::new(); // holds indices
for (i, &value) in nums.iter().enumerate() {
    if let Some(&front) = deque.front() {
        if front + k <= i { deque.pop_front(); }
    }
    while let Some(&back) = deque.back() {
        if nums[back] <= value { deque.pop_back(); } else { break; }
    }
    deque.push_back(i);
    if i + 1 >= k { result.push(nums[deque[0]]); }
}
```

`VecDeque` gives O(1) `push_back`, `pop_back`, and `pop_front`; a `Vec` cannot pop from the front in O(1). The loop maintains the invariant "indices in the deque are increasing, and the values they point to are strictly decreasing", so the front is always the window maximum.

**Gotcha.** Store **indices**, not values — you cannot tell whether the front has slid out of the window from its value alone. `deque[0]` indexes the deque by position (`Index<usize>` is implemented), while `nums[back]` indexes the input.

**Used in:** `sliding_window_maximum`.

#### Two counter maps and the `have`/`required` window

```rust
let mut need: HashMap<u8, usize> = HashMap::new();
for &b in t.as_bytes() { *need.entry(b).or_insert(0) += 1; }

let required = need.len();
let mut window: HashMap<u8, usize> = HashMap::new();
let mut have = 0usize;
// ...
if window.get(&b) == need.get(&b) { have += 1; }
```

`required` counts distinct characters, not total counts; `have` counts how many distinct needs are currently met. Comparing `Option<&usize>` with `==` works because `Option` implements `PartialEq`; `None == None` is `true`.

**Gotcha.** Increment `have` only when a count becomes *exactly* equal to its need, and decrement only when it drops *below* the need; otherwise the window appears satisfied too early. The comparison against `.get()` returns `Option<&usize>`, so a missing key on both sides does not count.

**Used in:** `minimum_window_substring`.

#### Backtracking with explicit `&mut` state

```rust
fn backtrack(row: usize, board: &mut Vec<Vec<char>>,
             cols: &mut Vec<bool>, diag1: &mut Vec<bool>, diag2: &mut Vec<bool>,
             results: &mut Vec<Vec<String>>) {
    if row == n { results.push(render(board)); return; }
    for col in 0..n {
        if cols[col] || diag1[row + n - 1 - col] || diag2[row + col] { continue; }
        board[row][col] = 'Q';
        cols[col] = true;
        // ...
        backtrack(row + 1, board, cols, diag1, diag2, results);
        board[row][col] = '.';
        cols[col] = false;
        // ...
    }
}
```

Every mutable structure is an explicit `&mut` parameter, and the undo lines mirror the choose lines. `vec![false; n]` and `vec![vec!['.'; n]; n]` build the state; `vec![...]` repeats a `Copy` value.

**Gotcha.** Nested functions cannot capture `n`; pass it or make it a constant. Recursion depth is `n`, so `n ≤ 9` is fine, but the number of boards is `n!` — the search is the cost, not the syntax.

**Used in:** `n_queens`.

#### Binary search with unsigned bounds and sentinels

```rust
let mut low = 0usize;
let mut high = m;
while low <= high {
    let i = (low + high) / 2;
    let j = half - i;
    let left1 = if i == 0 { i32::MIN } else { a[i - 1] };
    let right1 = if i == m { i32::MAX } else { a[i] };
    // ...
    if left1 <= right2 && left2 <= right1 { return /* ... */; }
    else if left1 > right2 { high = i - 1; }
    else { low = i + 1; }
}
unreachable!("input arrays must be sorted")
```

`i32::MIN`/`i32::MAX` are the sentinels that make the boundary checks uniform, so the loop needs no special cases for empty partitions. `(low + high) / 2` is safe here because the values are small; for `usize` near `MAX`, prefer `low + (high - low) / 2`.

**Gotcha.** `high = i - 1` would underflow if `i == 0`; it cannot here because the branch `left1 > right2` is false when `i == 0` (the left sentinel is `MIN`). Any change to the sentinels must preserve that argument.

**Used in:** `median_of_two_sorted_arrays`.

#### Borrow-checker patterns for linked state

```rust
// End the borrow before mutating:
let (prev, next) = (self.nodes[idx].prev, self.nodes[idx].next);

// Copy out of the map before touching self.nodes:
let idx = self.map.get(&key).copied();

// Mutate through one binding at a time:
if let Some(cnt) = window.get_mut(&lb) { *cnt -= 1; }
```

The rules to internalise: a `&` borrow forbids any `&mut` on the same value until it ends; `self.map` and `self.nodes` are *different fields*, so borrows of each can coexist, but a method call like `self.unlink(idx)` needs `&mut self` and therefore ends every outstanding borrow of `self`.

**Gotcha.** `self.nodes[idx].value = value; self.move_to_front(idx);` fails if an earlier `let node = &self.nodes[idx];` is still live. Copy the values you need (`let value = self.nodes[idx].value;`) or scope the borrow in a block.

**Used in:** `lru_cache`.

#### `loop` with `match` and or-patterns

```rust
loop {
    match (a, b) {
        (Some(x), Some(y)) => { /* ... */ }
        (None, rest) | (rest, None) => { *tail = rest; break; }
    }
}
```

`loop` repeats until a `break`; this one breaks with `()`, but `break value` is legal. Matching a tuple of two `Option`s with an or-pattern collapses the two "one side empty" arms into one.

**Used in:** `merge_k_sorted_lists`.

#### `f64` results and integer-to-float division

```rust
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    return if total % 2 == 1 {
        left1.max(left2) as f64
    } else {
        (left1.max(left2) + right1.min(right2)) as f64 / 2.0
    };
}
```

Casting to `f64` *before* dividing is mandatory: `(a + b) / 2` with integers truncates, and `(a + b) as f64 / 2.0` is the intended mean. `as f64` is a widening, exact conversion for `i32`.

**Gotcha.** `(left1 + right1) / 2` with negative odd sums truncates toward zero, not toward −∞. For a median that is acceptable because the sum is non-negative in the even-length case; in general, decide explicitly.

**Used in:** `median_of_two_sorted_arrays`.

#### Rendering with iterators

```rust
let solution: Vec<String> = board.iter()
    .map(|r| r.iter().collect::<String>())
    .collect();
```

`collect::<String>()` on `&char` works because `String: FromIterator<&char>`. This is the Rust equivalent of `board.map((r) => r.join(""))`.

**Used in:** `n_queens`.

#### Unreachable branches

```rust
unreachable!("input arrays must be sorted")
```

The Rust analogue of TS `throw new Error`: it documents an invariant and panics if it is violated. Use `debug_assert!` for checks you want only in development.

**Used in:** `median_of_two_sorted_arrays`, `merge_k_sorted_lists`.

### 4.3 Hard: cross-language notes

- TS `class` maps to Rust `struct` + `impl`. TS fields are `private` at compile time; Rust fields are module-private by default.
- Both express "no link" without null: TS uses `-1` (a sentinel) or a `LRUNode | undefined`; Rust uses `Option<usize>`. The Rust version makes the sentinel impossible to confuse with a real index.
- A TS linked list of objects would need `node.prev = otherNode` references; the repo's Rust and TS solutions both use **indices into an array** instead. The ownership model pushes Rust toward indices, and the TS solution follows the same design for symmetry and cache-friendly storage.
- TS binary search uses `Math.floor`; Rust uses integer division. Both are the same algorithm; only the rounding syntax differs.
- Sentinels are language-specific: `Infinity`/`-Infinity` in TS, `i32::MIN`/`i32::MAX`/`usize::MAX` in Rust. Choose one and state its meaning in a comment.

### 4.4 Hard readiness checklist

- [ ] Design a class/struct with a constructor and at least two private fields, and explain its invariant.
- [ ] Implement a doubly linked list by index, in both languages, with O(1) `unlink`/`link`.
- [ ] Write a monotonic `VecDeque` window and state why the front is the answer.
- [ ] Write a binary search with an explicit invariant and explain each boundary update.
- [ ] Implement backtracking with choose/undo symmetry, and name the failure mode if an undo is missing.
- [ ] Explain `.take()`, `Box`, and `&mut Option<Box<T>>` well enough to merge two linked lists.
- [ ] Explain why TS `!` and Rust `unwrap`/`unreachable!` are both assertions, and when each is justified.
- [ ] Convert between `i32`, `usize`, and `f64` at the right boundary and say why each cast is safe.

---

## 5. Cross-cutting failure modes

### 5.1 TypeScript strict-mode traps

| Symptom | Cause | Fix |
|---|---|---|
| `arr[i]` is possibly `'undefined'` | `noUncheckedIndexedAccess` | guard the index, or `!` after proving it |
| `Type 'undefined' is not assignable` | `exactOptionalPropertyTypes` treats `{ x?: T }` as "absent or T", not `T \| undefined` | omit the property, or type it `T \| undefined` explicitly |
| TS1484: a type must be imported using a type-only import | `verbatimModuleSyntax` | `import type { Foo } from "..."` |
| Numbers sort as strings | default `sort()` is lexicographic | pass `(a, b) => a - b` |
| A counter becomes `NaN` | reading `undefined` into arithmetic | `?? 0` at the read, or `!` only after a bounds proof |
| Shared inner arrays across rows | `new Array(n).fill([])` | `Array.from({ length: n }, () => [])` |
| `this` is `undefined` in a callback | method passed unbound | arrow function or `.bind(this)` |
| An `if` never narrows after `map.get` | the check is in a different scope | store the result in a `const` and check that |

### 5.2 Rust compiler errors, decoded

| Error | Message shape | Meaning | Fix |
|---|---|---|---|
| E0502 | "cannot borrow `x` as mutable because it is also borrowed as immutable" | a live `&` blocks a `&mut` | end the borrow (scope it, or copy the value out) |
| E0499 | "cannot borrow `x` as mutable more than once at a time" | two simultaneous `&mut` | sequence the mutations; use indices between them |
| E0382 | "borrow of moved value" | a `Vec`/`String` was passed by value | pass `&`/`&mut`, or `.clone()` |
| E0507 | "cannot move out of `x` which is behind a shared reference" | trying to move a field or value through `&` | `.clone()`, `.take()`, or restructure |
| E0308 | "mismatched types", often `usize` vs `i32` | no implicit numeric conversion | `as` cast at the boundary |
| E0596 | "cannot borrow as mutable, as it is not declared as mutable" | missing `mut` | `let mut` / `&mut` |
| E0277 | "trait bound not satisfied", often `Debug` | `println!("{:?}")` on a non-`Debug` type | `#[derive(Debug)]` |
| E0061 | "this function takes N arguments but M were supplied" | nested `fn` cannot capture | pass the captured values explicitly |
| E0505 | "cannot move out of `x` because it is borrowed" | move while a borrow is live | clone, or reorder so the borrow ends first |

---

## 6. Practice drills

Syntax becomes automatic by retrieval, not by reading. These drills are ordered by cost.

1. **Retype, don't copy.** Open a reference solution, close it, and retype it from memory. Compile. Fix only syntax errors first, then logic.
2. **Port one solution per tier.** Solve `two_sum` in TS then Rust, then `group_anagrams`, then `lru_cache`. The port forces you to confront the mapping table.
3. **Delete a line.** In a working solution, remove a `mut`, a `!`, or an `&`, predict the compiler's complaint, then compile. This calibrates your error intuition.
4. **Signature-only reps.** Write the function signature for five problems from the catalog in both languages without looking. This is the highest-yield five minutes at the start of a session.
5. **Explain the sentinel.** For `coin_change`, `median_of_two_sorted_arrays`, and `minimum_window_substring`, say aloud what "no answer" is represented by and why that value cannot collide with a real answer.
6. **State the invariant.** For `sliding_window_maximum`, `longest_valid_parentheses`, and `median_of_two_sorted_arrays`, write the loop invariant in one sentence before reading the code.
7. **Error-first reading.** When a solution surprises you, ask which Rust error or TS strict-mode diagnostic the alternative would produce. The language constraints often explain the design.

Run the drills against the worksheets in [`exercises/`](../exercises/) and check your work with `solutions/` and `cargo test`, as described in the [root README](../README.md#running-the-code).

---

## 7. References

**TypeScript**

- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html) — especially *Everyday Types*, *Narrowing*, *Classes*, and *Generics*.
- [`tsconfig` reference](https://www.typescriptlang.org/tsconfig) — `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`.
- [MDN JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript) — `Map`, `Set`, `Array`, `String`, `Number`.

**Rust**

- [The Rust Book](https://doc.rust-lang.org/book/) — chapters 3 (types), 4 (ownership), 6 (enums and `Option`), 8 (collections), 9 (errors), 10 (generics/traits/lifetimes), 15 (smart pointers).
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — `Option`, `match`, `HashMap`, `VecDeque`.
- [`std` documentation](https://doc.rust-lang.org/std/) — `Vec`, `VecDeque`, `HashMap`, `Option`, `String`.
- [Rustlings](https://github.com/rust-lang/rustlings) — ownership and collections exercises.

**Companion in this repo**

- [Problem-solving guide](problem-solving-guide.md) — how to approach, analyze, and test the problems.
- [Problems catalog](../problems/README.md) — the 33 problems, indexed by difficulty and topic.
