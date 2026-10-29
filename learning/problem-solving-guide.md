# How to Think About and Prepare for Algorithm Problems

This guide is a deep, opinionated orientation for the coding problems in this repository. It explains *how to think* — how to approach a problem, analyze it, and avoid the traps that waste interview time — rather than merely listing algorithms. Every claim is anchored to a well-credited source you can read independently; follow the links when you want more rigor than a paragraph can offer.

The method is language-agnostic. The concrete demonstrations use **TypeScript** and **Rust**, the two main languages of this repo (PHP and Python are present as lower-priority languages; React is a separate UI track).

---

## 1. Use a repeatable method, not intuition

Strong solvers do not jump straight to code. They run a disciplined loop, popularized by [George Pólya's *How to Solve It*](https://en.wikipedia.org/wiki/How_to_Solve_It) and adapted for interviews by [Gayle Laakmann McDowell in *Cracking the Coding Interview*](https://www.crackingthecodinginterview.com/):

1. **Understand** the problem.
2. **Devise a plan**.
3. **Carry out the plan**.
4. **Look back**.

The loop sounds trivial; the discipline is in *doing each phase before moving to the next*. Here is what each phase concretely demands.

### 1.1 Understand — don't solve the wrong problem

Before writing anything, pin down:

- **The exact inputs and outputs**, including types and units. "Return the indices" is a different problem from "return the two numbers."
- **The constraints.** They are hints, not fine print. `n ≤ 10⁴` permits O(n²); `n ≤ 10⁵` points to O(n log n); `n ≤ 10⁶` points to O(n) or O(n log n). A sorted input, a fixed alphabet, or "exactly one solution" all change the intended approach.
- **A worked example**, drawn by hand — small enough to trace, large enough to be non-trivial.
- **The single core question** underneath the wording. "Find the longest substring without repeating characters" is really "find the longest *window* whose characters are all distinct."

McDowell's book calls this the most skipped and most important phase; Pólya frames it as "what is the unknown? what are the data? what are the conditions?"

### 1.2 Plan — on paper, in English, with brute force first

Do not type during planning. Write the **brute force**, even if it is obviously too slow, because it (a) proves you understood the problem, (b) gives you a correct reference to test a faster idea against, and (c) makes the inefficiency visible.

Then ask a single productive question: *"What is wasteful about the brute force, and which data structure or technique removes that waste?"* If the brute force re-scans the array for each element, a hash map remembers it instead. If it recomputes the same subproblem, memoization caches it. Pólya's heuristics — **analogy** ("have I seen a related problem?"), **decomposition** ("can I split it into subproblems?"), **generalization/specialization** ("what if the input were smaller or simpler?") — are the classic tools for this step.

Finish by estimating the complexity *before* coding. If you cannot state the target time and space, you are not ready to type.

### 1.3 Carry out — implement faithfully, smallest first

Translate the plan, not a different idea you just had. Write the smallest version that compiles and runs, then add cases. Use descriptive names; a confusing variable is a bug waiting to happen. In Rust especially, "write the smallest thing that compiles" is practical advice, because the borrow checker forces you to clarify ownership early.

### 1.4 Look back — verify, then improve

Test the examples and the edge cases from §5. Walk a trace by hand. Only then ask: is it correct *for all inputs*? Can it be faster or use less space? Is it readable? Compare against the reference solution in this repo's `solutions/` and the doc's **Solution** section — but only *after* your own attempt.

---

## 2. Measure what you write: complexity and correctness

An algorithm is only worth anything if you can *say* how fast it is and *argue* that it is right. These two skills separate people who memorize answers from people who can solve new problems.

### 2.1 Big O, precisely

[Big O notation](https://en.wikipedia.org/wiki/Big_O_notation) describes how runtime or space **grows as the input grows**, ignoring constants and lower-order terms. Formally, `f(n) = O(g(n))` if there exist constants `c > 0` and `n₀` such that `f(n) ≤ c·g(n)` for all `n ≥ n₀`. In practice you rarely write the definition; you read it as "asymptotic upper bound."

The classes you must recognize instantly ([time complexity](https://en.wikipedia.org/wiki/Time_complexity)):

| Class | Name | Intuition | Repo example |
|---|---|---|---|
| O(1) | constant | independent of input size | array index, hash-map get |
| O(log n) | logarithmic | the search space halves each step | binary search (median-of-two-sorted-arrays) |
| O(n) | linear | each element touched a constant number of times | single pass, two pointers |
| O(n log n) | linearithmic | n passes, each halving | comparison sort (three_sum, merge_intervals) |
| O(n²) | quadratic | nested pass over n | brute-force pairs |
| O(n·k) | polynomial (2D) | depends on two inputs | coin_change (amount × coins) |
| O(2ⁿ) / O(n!) | exponential / factorial | each choice doubles, or permutes, the work | n_queens backtracking |

**How to read your own code:** sequential blocks *add*; nested loops *multiply*; a loop that halves its range each iteration contributes a `log n` factor. For recursion, the [master theorem](https://en.wikipedia.org/wiki/Master_theorem_(analysis_of_algorithms)) gives `T(n) = a·T(n/b) + f(n)` a closed form — the standard tool for divide-and-conquer like merge sort or merge-k-sorted-lists.

**Space** is the same idea applied to memory, and the distinction matters: *auxiliary space* (extra memory beyond the input) is what interviewers usually mean. An in-place algorithm uses O(1) auxiliary space even though the input itself occupies O(n).

**Amortized analysis** ([Wikipedia](https://en.wikipedia.org/wiki/Amortized_analysis)) averages the cost of one operation over a *sequence* — a dynamic array's push is occasionally O(n) (reallocation) but O(1) amortized, because the rare expensive step is paid back by many cheap ones.

### 2.2 Correctness: invariants and induction

Complexity says *how fast*; correctness says *why it works on every input*. The standard tool is the **loop invariant** ([Wikipedia](https://en.wikipedia.org/wiki/Loop_invariant)): a property that is true **before** the loop, remains true **after each iteration**, and, together with the loop's exit condition, implies the answer is right. The proof has three steps — *initialization*, *maintenance*, *termination* — and [CLRS (*Introduction to Algorithms*)](https://mitpress.mit.edu/9780262046305/introduction-to-algorithms/) is the canonical treatment, with a worked insertion-sort proof in its first chapters.

Consider this repo's two-pointer palindrome. The invariant is: *"the characters strictly outside `[left, right]` have already been confirmed to match."* It holds before the loop (nothing checked yet); each iteration either skips a non-alphanumeric or verifies one pair and moves inward, preserving the invariant; when `left ≥ right`, the whole string has been verified. That argument — not "it passed my test cases" — is what correctness means.

For a video treatment, [MIT OCW 6.006 (Introduction to Algorithms)](https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/) and Princeton's [Algorithms, Part I & II](https://www.coursera.org/learn/algorithms-part1) are excellent and free. Every problem document in this repo states a target complexity — confirm yours matches before you call it done.

---

## 3. Know your data structures

Algorithms are built from a small set of primitives. If you can state each structure's *superpower* — the one operation it makes cheap — you will usually know which one a problem wants.

| Structure | Cheap operations | Superpower | Reach for it when… |
|---|---|---|---|
| Array / `Vec` | index O(1) | random access by position | order matters, two-pointer, DP tables |
| Hash map | get/insert/delete O(1) | key → value, counting, membership | complement lookup (two_sum), frequency (group_anagrams, top_k_frequent) |
| Hash set | add/contains O(1) | deduplication, membership | contains_duplicate, sliding-window membership |
| Stack | push/pop O(1) | LIFO; "most recent unmatched" | valid_brackets, monotonic stack, longest_valid_parentheses |
| Queue | enqueue/dequeue O(1) | FIFO | breadth-first search |
| Deque | push/pop both ends O(1) | a window whose extremes you need | sliding_window_maximum (monotonic deque) |
| Linked list | insert/delete at a node O(1) | no shifting on removal | merge_k_sorted_lists, LRU cache |
| Heap / priority queue | min/max O(1), push/pop O(log n) | "the k smallest/largest" | top_k_frequent, merge_k_sorted_lists, Dijkstra |
| Union–Find | union/find ~O(α(n)) | connected components | islands (alternative to DFS) |
| Graph (adjacency list) | neighbors of a vertex | relationships | DFS/BFS over number_of_islands |

Two references cover this toolbox best: [Steven Skiena's *The Algorithm Design Manual*](https://www.algorist.com/) (a catalog of real problems mapped to the techniques that solve them) and [Sedgewick & Wayne's *Algorithms* (4th ed.)](https://algs4.cs.princeton.edu/) (rigorous, code-heavy, with a free companion course). For a free, concise algorithm reference, [cp-algorithms.com](https://cp-algorithms.com/) is the standard.

---

## 4. Recognize the pattern

Most interview problems are a known pattern in disguise. Recognition is most of the battle, and it comes from seeing the *tell* — the phrase or shape that gives the pattern away.

- **Hash map / set.** Tell: *"find a pair/target," "count occurrences," "is X present," "complement," "anagram."* → `two_sum`, `contains_duplicate`, `valid_anagram`, `group_anagrams`.
- **Two pointers.** Tell: *sorted input* plus *a pair/sum/target*, or *"meet in the middle."* → `valid_palindrome`, `reverse_string`, `container_with_most_water`, `three_sum`, `trapping_rain_water`.
- **Sliding window.** Tell: *contiguous subarray/substring* with a constraint that is *monotonic* (once a window is invalid, shrinking can only help). → `longest_substring_without_repeating_characters`, `minimum_window_substring`, `sliding_window_maximum`.
- **Prefix sums.** Tell: *range sums/queries* over an array. → the basis of `product_of_array_except_self` (product variant).
- **Binary search.** Tell: *sorted data* or a *monotonic predicate* (answers are "no, no, …, yes, yes"). → `median_of_two_sorted_arrays`.
- **Stack / monotonic stack.** Tell: *matching, nesting, "next greater/smaller,"* or *"most recent unmatched."* → `valid_brackets`, `longest_valid_parentheses`.
- **DFS / BFS.** Tell: *grid, graph, connected components, "reachable,"* shortest path in *unweighted* graph. → `number_of_islands`. (BFS for shortest, DFS for exploration/backtracking.)
- **Dynamic programming.** Tell: *"how many ways," "minimum/maximum cost,"* with **overlapping subproblems** and **optimal substructure**. Start with recursion + memoization, then convert to bottom-up. → `climbing_stairs`, `coin_change`, `maximum_subarray` (Kadane).
- **Greedy.** Tell: *"maximize/minimize"* where a **locally optimal choice** is provably globally optimal. → `best_time_to_buy_and_sell_stock`, parts of `container_with_most_water`.
- **Backtracking.** Tell: *"all combinations/permutations/placements"* — an exhaustive search with pruning. → `n_queens`.
- **Divide & conquer.** Tell: *"merge k sorted things," "closest pair,"* recursion that splits the input. → `merge_k_sorted_lists` (and `merge_intervals` splits via sorting).

The widely-cited [Blind 75](https://www.teamblind.com/post/New-Year-Gift---Curated-List-of-Top-75-LeetCode-Questions-to-Save-Your-Time-OaM1orEU), its popular successor [NeetCode](https://neetcode.io/), and [LeetCode's "Learn" tracks](https://leetcode.com/explore/learn/) are organized by exactly these patterns — use them to drill one pattern until you recognize it instantly.

---

## 5. Hunt edge cases and test like it ships

Interviews are lost on the boundary, not the happy path. Before coding, enumerate:

- **Empty and minimal:** `""`, `[]`, `n = 1`, one element.
- **Homogeneous:** all equal, all distinct, already sorted, reverse sorted.
- **Sign and overflow:** negative numbers, `INT_MIN`/`INT_MAX`, `k = 0` or `k > n` in rotations.
- **Boundaries and off-by-one:** first/last index, even vs. odd length, the exact threshold that flips a condition.
- **Validity of assumptions:** duplicates when the problem implies they may exist, empty subcases the statement doesn't forbid.

Then *encode them as tests*, not as afterthoughts. This repo treats tests as first-class: every Rust reference solution (in `solutions/`) carries a `#[cfg(test)]` block, and every TypeScript reference solution carries `console.log` checks. *Cracking the Coding Interview* has a dedicated chapter on testing and boundary-value analysis. Once you have a brute-force reference, a powerful trick is to **fuzz**: run the fast solution against the brute force on random small inputs and assert they agree — this catches the edge cases you failed to imagine.

---

## 6. When you're stuck: simplify, then recover

Being stuck is normal. The skill is a set of recovery moves:

- **Brute force first.** A correct O(n²) answer beats a clever broken one, and it is your oracle for testing the fast version.
- **Simplify the problem.** Drop a constraint, shrink the input, or solve for a special case, then generalize (Pólya's *"solve a related problem"*). For `trapping_rain_water`, the 1D two-pointer version is the simplification of the 2D one.
- **Change representation.** A different data structure or a sorted copy often makes the structure visible — sorting is the classic "preprocessing that reveals the pattern" (see `three_sum`, `merge_intervals`).
- **Trace by hand.** Step through a tiny input exactly, the way this repo's docs do in their trace tables. [Jon Bentley's *Programming Pearls*](https://en.wikipedia.org/wiki/Programming_Pearls) is the best book for this habit — it is essentially a collection of worked, hand-traced problems.
- **Find the invariant.** When a loop "feels" right but you cannot prove it, name the invariant; naming it usually exposes the bug.
- **Read solutions deliberately.** When you look at a reference, don't skim — find the *single key insight*, close it, and re-implement from memory. This converts "seeing" an answer into "owning" it.

When debugging an actual bug, reproduce it on the smallest failing input, then use `print`/`assert` to check your invariants at each step — the general discipline is well covered in *The Pragmatic Programmer* ([Hunt & Thomas](https://pragprog.com/titles/tpp20/the-pragmatic-programmer-20th-anniversary-edition/)) and in the [hand-traced style used throughout the problem docs](../problems/README.md).

---

## 7. Demonstration: two problems, two languages

The rest of this section shows the *same* algorithm expressed in TypeScript and Rust, and points out what the translation teaches.

### 7.1 Two Sum — hash map (complement lookup)

The insight: for each number, the value you need is `target - num`; store what you've seen and look the complement up in O(1) instead of scanning.

**TypeScript**

```typescript
function twoSum(nums: number[], target: number): number[] {
  const seen = new Map<number, number>(); // value -> index
  for (const [i, num] of nums.entries()) {
    const complement = target - num;
    const found = seen.get(complement);
    if (found !== undefined) {
      return [found, i];
    }
    seen.set(num, i);
  }
  return [];
}
```

**Rust**

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut seen: HashMap<i32, usize> = HashMap::new(); // value -> index
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }
    vec![]
}
```

The idea is identical; the languages differ in *how absence is expressed*. TypeScript's `Map.get` returns `number | undefined`, checked with `!== undefined`. Rust's `HashMap::get` returns `Option<&usize>`, checked with `if let Some(&j)`. This is the single most important TypeScript↔Rust correspondence: **`T | undefined` ⇄ `Option<T>`**. Two smaller notes: Rust's `usize` (its index type) must be cast to `i32` with `as` to match the return type; and `&num` in the loop pattern *dereferences* the borrowed `&i32` the iterator yields, so arithmetic works on a plain `i32`.

### 7.2 Valid Palindrome — two pointers

The insight: compare from both ends, skipping non-alphanumeric characters; if anything mismatches, it isn't a palindrome.

**TypeScript**

```typescript
function isPalindrome(s: string): boolean {
  const isAlnum = (c: string): boolean => /[a-z0-9]/i.test(c);
  let left = 0;
  let right = s.length - 1;
  while (left < right) {
    const l = s.charAt(left);
    const r = s.charAt(right);
    if (!isAlnum(l)) { left++;  continue; }
    if (!isAlnum(r)) { right--; continue; }
    if (l.toLowerCase() !== r.toLowerCase()) return false;
    left++;
    right--;
  }
  return true;
}
```

**Rust**

```rust
pub fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return true;
    }
    let mut left: usize = 0;
    let mut right: usize = bytes.len() - 1;
    while left < right {
        if !bytes[left].is_ascii_alphanumeric() { left += 1; continue; }
        if !bytes[right].is_ascii_alphanumeric() { right -= 1; continue; }
        if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
```

Three Rust ideas surface here that TypeScript hides:

- **`usize` cannot go negative.** `bytes.len() - 1` would underflow (panic) on an empty string, so the empty check exists *before* the subtraction — a guard TypeScript's `s.length - 1` never needs because its `number` is signed.
- **Strings are not indexable by character.** Rust's `&str` is UTF-8, so `s[left]` is illegal; `.as_bytes()` gives a `&[u8]` you *can* index, valid here because the problem is ASCII.
- **The final `true` has no `return` and no semicolon.** Rust's last expression is the return value; TypeScript requires an explicit `return`.

For the full line-by-line syntax breakdown of these and every other problem, open the relevant document from the [problem catalog](../problems/README.md) — each explains the TypeScript and Rust piece by piece.

### 7.3 A small translation cheat sheet

| Concept | TypeScript | Rust |
|---|---|---|
| "Maybe a value" | `T \| undefined` / `T \| null` | `Option<T>` (`Some` / `None`) |
| Lookup that may miss | `map.get(k)` → `undefined` | `map.get(&k)` → `Option<&V>` |
| Handle a miss | `if (found !== undefined)` | `if let Some(&v) = ...` |
| Index type | `number` (signed) | `usize` (unsigned, can't go below 0) |
| Mutability | `let` vs `const` | `let` (immutable) vs `let mut` |
| Pass/borrow to mutate | arrays are references | `&mut Vec<T>` (explicit borrow) |
| Return value | explicit `return` | last expression, no `;` |
| Type cast | `as Type` | `as Type` |
| Built-in map | `Map`, `Set` | `HashMap`, `HashSet` |
| Increment | `i++` | `i += 1` |

For a tier-by-tier treatment of this table — every construct from Easy (`Map` lookups, `usize` casts) through Medium (`entry` API, DP sentinels) to Hard (`VecDeque`, binary-search invariants) — see the [syntax guide](syntax-guide.md), which names the problems that exercise each feature.

**Language references:** [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html) and [MDN JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript) · [The Rust Book](https://doc.rust-lang.org/book/), [Rust by Example](https://doc.rust-lang.org/rust-by-example/), the [standard library](https://doc.rust-lang.org/std/), and [Rustlings](https://github.com/rust-lang/rustlings) · [PHP Manual](https://www.php.net/manual/) and [Python documentation](https://docs.python.org/3/) (lower priority) · [React documentation](https://react.dev/) (UI track).

---

## 8. Practice deliberately, not just more

Volume alone does not produce skill; *how* you practice does (see the research on [deliberate practice](https://en.wikipedia.org/wiki/Practice_(learning_method))).

- **Attempt first, review second.** Struggle with the blank worksheet in `exercises/` before opening `solutions/` or the doc's Solution section. The struggle is where learning happens.
- **Retrieve, don't re-read.** A week later, re-implement the problem from memory with no reference. Retrieval beats re-reading for long-term retention.
- **Interleave.** Mix patterns and difficulties in a session rather than batching one topic — mixing forces you to *recognize* the pattern instead of assuming it.
- **Trace and articulate.** You only understand an algorithm when you can trace it on a fresh example and state its invariant aloud.
- **Track progress.** Use [this repo's problem catalog](../problems/README.md), ordered by difficulty and topic, as your queue, and mark what you've completed.

---

## Reference list

**Books**

- Pólya, G. — *How to Solve It*. [Wikipedia](https://en.wikipedia.org/wiki/How_to_Solve_It)
- Cormen, Leiserson, Rivest, Stein — *Introduction to Algorithms* (CLRS). [MIT Press](https://mitpress.mit.edu/9780262046305/introduction-to-algorithms/)
- Skiena, S. — *The Algorithm Design Manual*. [algorist.com](https://www.algorist.com/)
- McDowell, G. L. — *Cracking the Coding Interview*. [crackingthecodinginterview.com](https://www.crackingthecodinginterview.com/)
- Sedgewick, R. & Wayne, K. — *Algorithms* (4th ed.). [algs4.cs.princeton.edu](https://algs4.cs.princeton.edu/)
- Bentley, J. — *Programming Pearls*. [Wikipedia](https://en.wikipedia.org/wiki/Programming_Pearls)
- Hunt, A. & Thomas, D. — *The Pragmatic Programmer*. [pragprog.com](https://pragprog.com/titles/tpp20/the-pragmatic-programmer-20th-anniversary-edition/)

**Courses & lectures**

- MIT OpenCourseWare 6.006 — *Introduction to Algorithms*. [ocw.mit.edu](https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/)
- Princeton — *Algorithms, Part I & II* (Sedgewick/Wayne). [Coursera](https://www.coursera.org/learn/algorithms-part1)

**Web references**

- Big O notation — [Wikipedia](https://en.wikipedia.org/wiki/Big_O_notation); Time complexity — [Wikipedia](https://en.wikipedia.org/wiki/Time_complexity)
- Master theorem — [Wikipedia](https://en.wikipedia.org/wiki/Master_theorem_(analysis_of_algorithms)); Amortized analysis — [Wikipedia](https://en.wikipedia.org/wiki/Amortized_analysis); Loop invariant — [Wikipedia](https://en.wikipedia.org/wiki/Loop_invariant)
- *Algorithms for Competitive Programming* — [cp-algorithms.com](https://cp-algorithms.com/)
- Laaksonen, A. — *Competitive Programmer's Handbook* (free). [cses.fi](https://cses.fi/book/book.pdf)
- LeetCode Learn — [leetcode.com/explore/learn](https://leetcode.com/explore/learn/)
- Blind 75 — [original curated list](https://www.teamblind.com/post/New-Year-Gift---Curated-List-of-Top-75-LeetCode-Questions-to-Save-Your-Time-OaM1orEU)
- NeetCode — [neetcode.io](https://neetcode.io/)
- USACO Guide — [usaco.guide](https://usaco.guide/)

**Language documentation**

- TypeScript Handbook — [typescriptlang.org](https://www.typescriptlang.org/docs/handbook/intro.html)
- MDN JavaScript — [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Web/JavaScript)
- The Rust Book — [doc.rust-lang.org/book](https://doc.rust-lang.org/book/)
- Rust by Example — [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/)
- Rust standard library — [doc.rust-lang.org/std](https://doc.rust-lang.org/std/)
- Rustlings — [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
- PHP Manual — [php.net](https://www.php.net/manual/)
- Python documentation — [docs.python.org](https://docs.python.org/3/)
- React documentation — [react.dev](https://react.dev/)
