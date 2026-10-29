# Problems Catalog

An indexed catalog of every coding exercise documented in this directory. Each entry links to a self-contained markdown doc that follows the same structure:

1. **Problem** — statement, examples, constraints, and "things to think about" before coding.
2. **Solution** — approach, TypeScript, and Rust implementations, plus likely interviewer follow-ups.
3. **Syntax Explanation** — the algorithm in plain steps, each language explained piece by piece, a worked trace, and a TypeScript↔Rust comparison table.

Blank worksheets live under `../exercises/` and reference solutions under `../solutions/`, in **TypeScript**, **Rust**, **PHP**, and **Python** (PHP and Python are lower-priority languages). See the [root README](../README.md#running-the-code) for how to run each. A separate [React component-exercise track](../README.md#react) lives in `../exercises/react/` and `../solutions/react/`.

> **How to practice:** read the Problem section, attempt your own solution, then scroll to the Solution and Syntax Explanation to compare and learn. Track which ones you've done in the checkbox column.

> **New to this?** Start with the [learning guide](../learning/problem-solving-guide.md) on how to think about and prepare for these problems, then use the [syntax guide](../learning/syntax-guide.md) to check the TypeScript and Rust language features each difficulty tier assumes.

## Difficulty summary

| Difficulty | Count |
|---|---|
| Easy | 13 |
| Medium | 12 |
| Hard | 8 |
| **Total** | **33** |

---

## Easy

| # | Problem | Topics | Complexity | Doc |
|---|---|---|---|---|
| 1 | Two Sum | hash map, complement lookup | O(n) / O(n) | [easy/two_sum.md](easy/two_sum.md) |
| 2 | Valid Brackets | stack, matching pairs | O(n) / O(n) | [easy/valid_brackets.md](easy/valid_brackets.md) |
| 3 | Valid Palindrome | two pointers, ASCII filtering | O(n) / O(1) | [easy/valid_palindrome.md](easy/valid_palindrome.md) |
| 4 | Reverse String | two pointers, in-place swap | O(n) / O(1) | [easy/reverse_string.md](easy/reverse_string.md) |
| 5 | Contains Duplicate | hash set | O(n) / O(n) | [easy/contains_duplicate.md](easy/contains_duplicate.md) |
| 6 | Valid Anagram | character counting | O(n) / O(1) | [easy/valid_anagram.md](easy/valid_anagram.md) |
| 7 | First Unique Character | frequency map | O(n) / O(1) | [easy/first_unique_character.md](easy/first_unique_character.md) |
| 8 | Single Number | bitwise XOR | O(n) / O(1) | [easy/single_number.md](easy/single_number.md) |
| 9 | Best Time to Buy and Sell Stock | greedy, running min | O(n) / O(1) | [easy/best_time_to_buy_and_sell_stock.md](easy/best_time_to_buy_and_sell_stock.md) |
| 10 | Plus One | array, carry propagation | O(n) / O(1) | [easy/plus_one.md](easy/plus_one.md) |
| 11 | Climbing Stairs | dynamic programming, Fibonacci | O(n) / O(1) | [easy/climbing_stairs.md](easy/climbing_stairs.md) |
| 12 | Roman to Integer | hash map, lookahead | O(n) / O(1) | [easy/roman_to_integer.md](easy/roman_to_integer.md) |
| 13 | Remove Duplicates from Sorted Array | two pointers, in-place | O(n) / O(1) | [easy/remove_duplicates_from_sorted_array.md](easy/remove_duplicates_from_sorted_array.md) |

## Medium

| # | Problem | Topics | Complexity | Doc |
|---|---|---|---|---|
| 14 | Longest Substring Without Repeating Characters | sliding window, hash set | O(n) / O(min(n, Σ)) | [medium/longest_substring_without_repeating_characters.md](medium/longest_substring_without_repeating_characters.md) |
| 15 | Longest Palindromic Substring | expand around center | O(n²) / O(1) | [medium/longest_palindromic_substring.md](medium/longest_palindromic_substring.md) |
| 16 | Group Anagrams | hash map, sorting/counting | O(n·k log k) / O(n·k) | [medium/group_anagrams.md](medium/group_anagrams.md) |
| 17 | Top K Frequent Elements | bucket sort / heap | O(n) / O(n) | [medium/top_k_frequent_elements.md](medium/top_k_frequent_elements.md) |
| 18 | Product of Array Except Self | prefix/suffix products | O(n) / O(1) | [medium/product_of_array_except_self.md](medium/product_of_array_except_self.md) |
| 19 | Maximum Subarray | Kadane's algorithm | O(n) / O(1) | [medium/maximum_subarray.md](medium/maximum_subarray.md) |
| 20 | Merge Intervals | sorting, interval merging | O(n log n) / O(n) | [medium/merge_intervals.md](medium/merge_intervals.md) |
| 21 | Rotate Array | array reversal | O(n) / O(1) | [medium/rotate_array.md](medium/rotate_array.md) |
| 22 | 3Sum | sorting, two pointers | O(n²) / O(1) | [medium/three_sum.md](medium/three_sum.md) |
| 23 | Container With Most Water | two pointers, greedy | O(n) / O(1) | [medium/container_with_most_water.md](medium/container_with_most_water.md) |
| 24 | Number of Islands | graph DFS/BFS, flood fill | O(r·c) / O(r·c) | [medium/number_of_islands.md](medium/number_of_islands.md) |
| 25 | Coin Change | dynamic programming | O(amount·coins) / O(amount) | [medium/coin_change.md](medium/coin_change.md) |

## Hard

| # | Problem | Topics | Complexity | Doc |
|---|---|---|---|---|
| 26 | Median of Two Sorted Arrays | binary search on partitions | O(log(min(m,n))) / O(1) | [hard/median_of_two_sorted_arrays.md](hard/median_of_two_sorted_arrays.md) |
| 27 | Trapping Rain Water | two pointers, running max | O(n) / O(1) | [hard/trapping_rain_water.md](hard/trapping_rain_water.md) |
| 28 | Longest Valid Parentheses | stack of indices | O(n) / O(n) | [hard/longest_valid_parentheses.md](hard/longest_valid_parentheses.md) |
| 29 | Sliding Window Maximum | monotonic deque | O(n) / O(k) | [hard/sliding_window_maximum.md](hard/sliding_window_maximum.md) |
| 30 | Minimum Window Substring | sliding window, two maps | O(m+n) / O(Σ) | [hard/minimum_window_substring.md](hard/minimum_window_substring.md) |
| 31 | LRU Cache | hash map + linked list | O(1) / O(capacity) | [hard/lru_cache.md](hard/lru_cache.md) |
| 32 | N-Queens | backtracking | O(n!) / O(n) | [hard/n_queens.md](hard/n_queens.md) |
| 33 | Merge k Sorted Lists | divide & conquer / heap | O(n·log k) / O(1) | [hard/merge_k_sorted_lists.md](hard/merge_k_sorted_lists.md) |

---

## By topic

- **Arrays & hashing:** Two Sum, Contains Duplicate, Valid Anagram, Group Anagrams, Top K Frequent Elements, Product of Array Except Self, First Unique Character, Single Number.
- **Two pointers:** Valid Palindrome, Reverse String, Remove Duplicates from Sorted Array, Container With Most Water, 3Sum, Trapping Rain Water.
- **Sliding window:** Longest Substring Without Repeating Characters, Minimum Window Substring, Sliding Window Maximum.
- **Stack / monotonic stack:** Valid Brackets, Longest Valid Parentheses.
- **Dynamic programming:** Climbing Stairs, Coin Change, Maximum Subarray.
- **Greedy:** Best Time to Buy and Sell Stock, Maximum Subarray.
- **Intervals:** Merge Intervals.
- **Graph / matrix:** Number of Islands.
- **Binary search:** Median of Two Sorted Arrays.
- **Linked list:** Merge k Sorted Lists.
- **Design:** LRU Cache.
- **Backtracking:** N-Queens.
- **Bit manipulation:** Single Number.

## Suggested progression

1. Start with the Easy **hash map / two pointers** problems (Two Sum → Valid Palindrome → Reverse String → Contains Duplicate).
2. Move to **stack** (Valid Brackets), then the **sliding window** trio at Medium.
3. Tackle **dynamic programming** (Climbing Stairs → Maximum Subarray → Coin Change).
4. Finish with the **Hard** set, which mostly recombines Medium techniques under tighter constraints.
