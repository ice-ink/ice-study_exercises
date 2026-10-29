# Merge K Sorted Lists (deepseek generated)

## Problem

You are given an array of `k` linked lists, each sorted in ascending order. Merge all the lists into one sorted linked list and return its head. The merge must reuse the existing nodes (no copying of values into new nodes), and an empty list (or an empty input array) produces `null`/`None`.

## Examples

| Input | Output | Why |
|---|---|---|
| `lists = [[1,4,5],[1,3,4],[2,6]]` | `[1,1,2,3,4,4,5,6]` | Merge of three sorted lists |
| `lists = []` | `[]` | No lists at all |
| `lists = [[]]` | `[]` | One empty list |
| `lists = [[1,2,3]]` | `[1,2,3]` | Single list is already the answer |
| `lists = [[], [1], []]` | `[1]` | Empty lists are ignored |

## Constraints

- `k == lists.length`
- `0 <= k <= 10^4`
- `0 <= lists[i].length <= 500`
- `-10^4 <= lists[i][j] <= 10^4`
- Each `lists[i]` is sorted ascending; the total number of nodes is at most `10^4`.

## Things to think about before coding

- Merging list-by-list into an accumulator makes early lists get re-merged over and over. What is that time complexity, and where does the waste come from?
- How does pairing lists up in a tournament bracket reduce the number of times any given node is compared?
- You already know how to merge *two* sorted lists in O(n) time with a dummy head. How do you reuse that helper?
- Target O(n·log k) time and O(1) extra space.

Give it a try before scrolling further.

---

## Solution

### Approach

Use **divide and conquer**: repeatedly merge pairs of lists, halving the number of lists each round, until one list remains. Each round reuses a `mergeTwoLists` helper that splices two sorted lists together with a dummy head and a tail pointer, in O(n) time and O(1) extra space.

1. If there are no lists, return `null`/`None`.
2. With an `interval` starting at 1, merge `lists[i]` with `lists[i + interval]`, then double `interval` and repeat.
3. After the loop, `lists[0]` holds the fully merged list.

**Time:** O(n·log k), every node is visited once per round and there are O(log k) rounds. **Space:** O(1) extra — only a few pointers/index variables.

### TypeScript

```typescript
class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val: number = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

function mergeTwoLists(a: ListNode | null, b: ListNode | null): ListNode | null {
  const dummy = new ListNode(0);
  let tail: ListNode = dummy;

  while (a !== null && b !== null) {
    if (a.val <= b.val) {
      tail.next = a;
      a = a.next;
    } else {
      tail.next = b;
      b = b.next;
    }
    tail = tail.next!;
  }

  tail.next = a ?? b;
  return dummy.next;
}

function mergeKLists(lists: Array<ListNode | null>): ListNode | null {
  if (lists.length === 0) return null;

  // Repeatedly merge pairs (divide and conquer) until one list remains.
  let interval = 1;
  while (interval < lists.length) {
    for (let i = 0; i + interval < lists.length; i += interval * 2) {
      lists[i] = mergeTwoLists(lists[i]!, lists[i + interval]!);
    }
    interval *= 2;
  }

  return lists[0]!;
}

// Quick checks
function makeList(values: number[]): ListNode | null {
  let head: ListNode | null = null;
  for (let i = values.length - 1; i >= 0; i--) {
    head = new ListNode(values[i]!, head);
  }
  return head;
}

function listToArray(head: ListNode | null): number[] {
  const result: number[] = [];
  let current = head;
  while (current !== null) {
    result.push(current.val);
    current = current.next;
  }
  return result;
}

console.log(listToArray(mergeKLists([makeList([1, 4, 5]), makeList([1, 3, 4]), makeList([2, 6])]))); // [1,1,2,3,4,4,5,6]
console.log(listToArray(mergeKLists([]))); // []
console.log(listToArray(mergeKLists([makeList([])]))); // []
```

### Rust

```rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn merge_two_lists(
    mut a: Option<Box<ListNode>>,
    mut b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;

    loop {
        match (a, b) {
            (Some(x), Some(y)) => {
                if x.val <= y.val {
                    let mut x = x;
                    let rest = x.next.take();
                    *tail = Some(x);
                    a = rest;
                    b = Some(y);
                } else {
                    let mut y = y;
                    let rest = y.next.take();
                    *tail = Some(y);
                    a = Some(x);
                    b = rest;
                }
                tail = match tail {
                    Some(node) => &mut node.next,
                    None => unreachable!(),
                };
            }
            (None, rest) | (rest, None) => {
                *tail = rest;
                break;
            }
        }
    }

    head
}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }

    // Repeatedly merge pairs (divide and conquer) until one list remains.
    let mut interval = 1usize;
    while interval < lists.len() {
        let mut i = 0usize;
        while i + interval < lists.len() {
            let left = lists[i].take();
            let right = lists[i + interval].take();
            lists[i] = merge_two_lists(left, right);
            i += interval * 2;
        }
        interval *= 2;
    }

    lists[0].take()
}

fn list_from(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let lists = vec![
            list_from(&[1, 4, 5]),
            list_from(&[1, 3, 4]),
            list_from(&[2, 6]),
        ];
        assert_eq!(
            list_to_vec(merge_k_lists(lists)),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(merge_k_lists(vec![]), None);
    }

    #[test]
    fn single_list() {
        let lists = vec![list_from(&[1, 2, 3])];
        assert_eq!(list_to_vec(merge_k_lists(lists)), vec![1, 2, 3]);
    }

    #[test]
    fn some_empty_lists() {
        let lists = vec![
            list_from(&[]),
            list_from(&[1, 2]),
            list_from(&[]),
            list_from(&[3]),
        ];
        assert_eq!(list_to_vec(merge_k_lists(lists)), vec![1, 2, 3]);
    }
}
```

### Notes on the two implementations

- TypeScript's `mergeTwoLists` uses a `dummy` node on the stack and a non-null `tail`; Rust uses an `Option<Box<ListNode>>` cursor so each "next" slot can be reassigned without fighting the borrow checker.
- Rust moves the `next` link out with `.take()` (leaving `None` in its place) before handing the node to the output, avoiding a partial move.
- Neither implementation allocates new payload nodes for the merge — they only rewire `next` pointers (TypeScript) or move `Box<ListNode>` values (Rust).

## Likely interviewer follow-ups

1. **Heap approach:** how would a min-heap of size `k` solve this, and what is its space complexity versus divide-and-conquer?
2. **Stability:** are equal values kept in their original relative order, and does the `<=` comparison matter for that?
3. **Iterative vs recursive merge:** what is the space difference between a recursive and an iterative `mergeTwoLists`?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. If the input is empty, return nothing.
2. Start with an `interval` of 1.
3. Merge `lists[i]` with `lists[i + interval]` for every valid `i`, storing the result back in `lists[i]`.
4. Double `interval` and repeat until only `lists[0]` is left.

## TypeScript, piece by piece

```typescript
class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val: number = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}
```
A linked-list node holds a value and a reference to the next node (or `null` at the tail). The default constructor parameters let callers write `new ListNode(5)` or `new ListNode(5, someNext)`, and assign both fields so strict mode is satisfied.

```typescript
const dummy = new ListNode(0);
let tail: ListNode = dummy;
```
`dummy` is a throwaway head; `tail` always points at the last node of the output so far. Starting both at `dummy` means the real result is `dummy.next`, and we never need a special case for "first node".

```typescript
while (a !== null && b !== null) {
  if (a.val <= b.val) { tail.next = a; a = a.next; }
  else                { tail.next = b; b = b.next; }
  tail = tail.next!;
}
```
We always link the smaller head, then advance that list's head. `tail = tail.next!` moves the tail forward; the `!` asserts the slot we just filled is non-null (it is, because we set it to a non-null `a` or `b`).

```typescript
let interval = 1;
while (interval < lists.length) {
  for (let i = 0; i + interval < lists.length; i += interval * 2) {
    lists[i] = mergeTwoLists(lists[i]!, lists[i + interval]!);
  }
  interval *= 2;
}
```
`interval` is the distance between the two lists being merged. Merging `(i, i+interval)` and stepping by `2 * interval` forms a tournament bracket: with `k = 3`, round 1 merges `(0,1)`, round 2 merges `(0,2)`. `lists[i]!` and `lists[i + interval]!` are non-null assertions because the loop condition guarantees those indices exist.

## Rust, piece by piece

```rust
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```
`Option<Box<ListNode>>` is how Rust expresses a nullable owned pointer: `Box` owns the node on the heap, and `Option` lets the end of the list be `None`. Every node has exactly one owner, which is why the merge can move `Box`es around without copying data.

```rust
let mut head: Option<Box<ListNode>> = None;
let mut tail = &mut head;
```
`tail` is a mutable reference to an `Option<Box<ListNode>>` slot. Writing `*tail = Some(node)` links the next node into whatever slot `tail` currently points at, and later `tail = &mut node.next` moves the cursor to the node's own "next" slot.

```rust
let mut x = x;
let rest = x.next.take();
*tail = Some(x);
```
`x.next.take()` replaces `x.next` with `None` and returns the old value, so we "split off" the rest of the list. We then move the now-detached `x` (whose `next` is `None`) into the output, and continue with `rest`. `Option::take` is the idiomatic way to move out of an `Option` you still need to keep valid.

```rust
tail = match tail {
    Some(node) => &mut node.next,
    None => unreachable!(),
};
```
After `*tail = Some(x)` the cursor's slot is occupied, so `Some(node)` matches and we advance to `&mut node.next`. The `None => unreachable!()` arm documents that this is impossible; `unreachable!` produces the `!` type that coerces to any type, including `&mut Option<Box<ListNode>>`.

## Trace: `lists = [[1,4,5],[1,3,4],[2,6]]` (should be `[1,1,2,3,4,4,5,6]`)

Round 1 (`interval = 1`) merges `[1,4,5]` with `[1,3,4]`:

| Step | a head | b head | pick | merged so far |
|---|---|---|---|---|
| 1 | 1 | 1 | a (1 ≤ 1) | `[1]` |
| 2 | 4 | 1 | b (4 > 1) | `[1,1]` |
| 3 | 4 | 3 | b (4 > 3) | `[1,1,3]` |
| 4 | 4 | 4 | a (4 ≤ 4) | `[1,1,3,4]` |
| 5 | 5 | 4 | b (5 > 4) | `[1,1,3,4,4]` |
| 6 | 5 | null | append a | `[1,1,3,4,4,5]` |

Round 2 (`interval = 2`) merges `[1,1,3,4,4,5]` with `[2,6]` to produce `[1,1,2,3,4,4,5,6]`, which is returned.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Nullable node link | `next: ListNode \| null` | `next: Option<Box<ListNode>>` |
| Owned heap node | object reference (GC) | `Box<ListNode>` |
| Dummy head | `new ListNode(0)` | `let mut head = None` + cursor |
| Advance tail | `tail = tail.next!` | `tail = &mut node.next` |
| Take a value out | `lists[i]!` (copy of ref) | `lists[i].take()` (moves the `Option`) |
| Merge helper | returns `ListNode \| null` | returns `Option<Box<ListNode>>` |
