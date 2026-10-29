# Group Anagrams (deepseek generated)

## Problem

Given an array of strings `strs`, group the strings that are **anagrams** of each other (strings made of exactly the same letters, in any order). Return the groups in any order; the strings within each group may also be in any order.

## Examples

| Input | Output | Why |
|---|---|---|
| `["eat","tea","tan","ate","nat","bat"]` | `[["eat","tea","ate"],["tan","nat"],["bat"]]` | The first three share letters `a,e,t`; the next two share `a,n,t`; `bat` is alone |
| `[""]` | `[[""]]` | A single empty string is its own group |
| `["a"]` | `[["a"]]` | One string, one group |
| `["ab","ba","abc"]` | `[["ab","ba"],["abc"]]` | `ab`/`ba` are anagrams; `abc` has an extra letter |
| `["a","a"]` | `[["a","a"]]` | Duplicate strings are still anagrams of each other |

## Constraints

- `1 <= strs.length <= 10^4`
- `0 <= strs[i].length <= 100`
- `strs[i]` consists of lowercase English letters.

## Things to think about before coding

- Two strings are anagrams exactly when their **sorted** letters are equal. What does that suggest as a grouping key?
- Can you build the key in O(k log k) by sorting, or in O(k) by counting each of the 26 letters?
- What data structure maps a key to a growing list of strings?
- Be careful to handle the empty string `""` — it sorts to `""` and groups only with other empty strings.

Give it a try before scrolling further.

---

## Solution

### Approach

Give every string a canonical key that is the same for all of its anagrams. The simplest key is the string's characters **sorted**. Use a hash map from that key to a list of the original strings, then return all the lists.

1. Create a map from sorted-key string → list of strings.
2. For each string, sort its characters to build the key.
3. Append the original string to the list stored under that key (creating the list if needed).
4. Return the map's values as a list of lists.

**Time:** O(n · k log k), where `n` is the number of strings and `k` the max length (sorting each string). **Space:** O(n · k) for the stored strings.

### TypeScript

```typescript
function groupAnagrams(strs: string[]): string[][] {
  const groups = new Map<string, string[]>();

  for (const str of strs) {
    const key = str.split("").sort().join("");
    const existing = groups.get(key);
    if (existing !== undefined) {
      existing.push(str);
    } else {
      groups.set(key, [str]);
    }
  }

  return Array.from(groups.values());
}

// Quick checks
console.log(groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]));
// [["eat","tea","ate"],["tan","nat"],["bat"]]
console.log(groupAnagrams([""]));  // [[""]]
console.log(groupAnagrams(["a"])); // [["a"]]
console.log(groupAnagrams(["ab", "ba", "abc"])); // [["ab","ba"],["abc"]]
```

### Rust

```rust
use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        groups.entry(key).or_default().push(s);
    }

    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut result = group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        for group in &mut result {
            group.sort();
        }
        result.sort();

        assert_eq!(
            result,
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
            ]
        );

        assert_eq!(group_anagrams(vec!["".to_string()]), vec![vec!["".to_string()]]);
        assert_eq!(group_anagrams(vec!["a".to_string()]), vec![vec!["a".to_string()]]);
    }
}
```

### Notes on the two implementations

- Both use the sorted-string key; the map values keep the original strings so the answer preserves the inputs.
- TypeScript mutates in place with `.sort()` and then joins; Rust collects to a `Vec<char>`, uses `sort_unstable`, and collects back into a `String`.
- The Rust test sorts both the groups and the strings inside each group before comparing, because `HashMap` iteration order is unspecified.

## Likely interviewer follow-ups

1. **O(n·k) key:** how would you key by a 26-element letter-count vector instead of sorting? Trade-off?
2. **Order guarantee:** what would you change to return groups in the order their first member appeared?
3. **Large strings:** when is counting faster than sorting, and vice versa?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Decide on a "canonical form" that all anagrams share — sorting the letters works.
2. Walk through the input, compute each string's key.
3. Store each string in a bucket addressed by its key.
4. At the end, hand back all the buckets.

## TypeScript, piece by piece

```typescript
const groups = new Map<string, string[]>();
```
A `Map` from a string key to an array of strings. We will use the sorted letters as the key and the group members as the value.

```typescript
const key = str.split("").sort().join("");
```
Three chained calls: `split("")` turns the string into an array of single characters; `.sort()` sorts that array in place (lexicographically by default); `.join("")` glues the characters back into a string. `"eat"` becomes `"aet"`.

```typescript
const existing = groups.get(key);
if (existing !== undefined) {
  existing.push(str);
} else {
  groups.set(key, [str]);
}
```
`groups.get(key)` returns the bucket or `undefined` when the key is new. If it exists we push into the same array (so the reference stored in the map grows); otherwise we create a new array `[str]` under that key.

```typescript
return Array.from(groups.values());
```
`groups.values()` is an *iterator* over the buckets, not an array. `Array.from(...)` materializes it into a real `string[][]`, which is the declared return type.

## Rust, piece by piece

```rust
let mut groups: HashMap<String, Vec<String>> = HashMap::new();
```
A `HashMap` keyed by `String` (the sorted letters) whose values are `Vec<String>` buckets. `mut` is needed because we keep inserting into it.

```rust
let mut chars: Vec<char> = s.chars().collect();
chars.sort_unstable();
```
`s.chars()` iterates over the characters and `.collect()` gathers them into a `Vec<char>`. `sort_unstable()` sorts in place — "unstable" just means it may not preserve the order of equal elements, which is fine here and slightly faster than `sort()`.

```rust
let key: String = chars.into_iter().collect();
```
`into_iter()` consumes the `Vec<char>` and `.collect()` rebuilds a `String` from the characters. The `: String` annotation tells `collect` what to produce (it could otherwise be inferred as `Vec<char>` again).

```rust
groups.entry(key).or_default().push(s);
```
This is Rust's **entry API**. `groups.entry(key)` returns a slot that may or may not exist. `.or_default()` gives a `&mut Vec<String>`, creating an empty `Vec` first if the key is absent. `.push(s)` then appends the original string — all in one expression, no `if let` needed.

```rust
groups.into_values().collect()
```
`into_values()` yields the map's values (the buckets) by moving them out of the map, and `.collect()` gathers them into the return type `Vec<Vec<String>>`. Using `into_values` (rather than `values`) avoids cloning.

## Trace: `["eat","tea","tan","ate","nat","bat"]`

| Step | str | sorted key | groups (key → values) |
|---|---|---|---|
| 1 | `eat` | `aet` | `aet: [eat]` |
| 2 | `tea` | `aet` | `aet: [eat, tea]` |
| 3 | `tan` | `ant` | `aet: [eat, tea]`, `ant: [tan]` |
| 4 | `ate` | `aet` | `aet: [eat, tea, ate]`, `ant: [tan]` |
| 5 | `nat` | `ant` | `aet: [eat, tea, ate]`, `ant: [tan, nat]` |
| 6 | `bat` | `abt` | `aet: [eat, tea, ate]`, `ant: [tan, nat]`, `abt: [bat]` |

The final answer is the three value lists: `[["eat","tea","ate"],["tan","nat"],["bat"]]`.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Hash map | `Map<string, string[]>` | `HashMap<String, Vec<String>>` |
| Sort characters | `str.split("").sort().join("")` | `chars.sort_unstable()` after `collect()` |
| Get or create bucket | explicit `if` on `.get` | `entry(k).or_default()` |
| Append to bucket | `existing.push(str)` | `.push(s)` on the `&mut` |
| Map values → array/vec | `Array.from(map.values())` | `map.into_values().collect()` |
| "Maybe a value" | `T \| undefined` | `Option<T>` (avoided via entry API) |
| Character list | `str.split("")` → `string[]` | `s.chars().collect()` → `Vec<char>` |
