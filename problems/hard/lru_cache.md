# LRU Cache (deepseek generated)

## Problem

Design a least-recently-used (LRU) cache. `get(key)` returns the cached value if the key exists (and marks it most-recently-used), otherwise `-1`. `put(key, value)` inserts or updates the entry (and marks it most-recently-used); if the cache exceeds its `capacity`, it evicts the least-recently-used entry. Both operations must run in O(1) time.

## Examples

For a cache with `capacity = 2`:

| Operations | Result | Why |
|---|---|---|
| `put(1,1); put(2,2); get(1)` | `1` | `get` refreshes recency |
| …then `put(3,3); get(2)` | `-1` | key 2 was the LRU and got evicted |
| …then `put(4,4); get(1)` | `-1` | key 1 is now the LRU and got evicted |
| …then `get(3); get(4)` | `3`, `4` | both are still cached |
| `put(1,10); put(1,20); get(1)` | `20` | an update overwrites without evicting |

## Constraints

- `1 <= capacity <= 3000`
- `0 <= key <= 10^4`
- `0 <= value <= 10^5`
- At most `2 * 10^5` calls will be made to `get` and `put`.

## Things to think about before coding

- A hash map gives O(1) lookup but no ordering; a linked list gives ordering but O(n) lookup. How do you combine them so both operations are O(1)?
- "Most recently used" is one end of the list; "least recently used" is the other. Which end should eviction read?
- Updating an existing key must not change the number of entries. What does that imply for your insert path?
- In Rust, a classic `Rc<RefCell<Node>>` list is awkward. Can you represent a linked list with array indices instead of pointers?

Give it a try before scrolling further.

---

## Solution

### Approach

Store every node in a `Vec` and build a doubly linked list out of `prev`/`next` **indices** into that `Vec`. A `HashMap<key, index>` gives O(1) lookup of where a key lives. The list is ordered most-recently-used (front) to least-recently-used (tail).

1. `get`: look up the key's index; if missing return `-1`, otherwise return the value and move the node to the front.
2. `put`: if the key exists, update its value and move it to the front. If it is new, push a node if there is room, else reuse the tail node (evict its old key); then link the node at the front.
3. Moving a node is a constant number of index rewrites: unlink it from its neighbors, then link it at the head.

**Time:** O(1) for `get` and `put`. **Space:** O(capacity), because a reused tail slot never grows the `Vec`.

### TypeScript

```typescript
interface LRUNode {
  key: number;
  value: number;
  prev: number; // index into nodes, -1 means "none"
  next: number; // index into nodes, -1 means "none"
}

class LRUCache {
  private capacity: number;
  private map: Map<number, number>; // key -> index into nodes
  private nodes: LRUNode[]; // storage; slots are never removed, only reused
  private head: number; // index of the most-recently-used node, -1 if empty
  private tail: number; // index of the least-recently-used node, -1 if empty

  constructor(capacity: number) {
    this.capacity = capacity;
    this.map = new Map();
    this.nodes = [];
    this.head = -1;
    this.tail = -1;
  }

  get(key: number): number {
    const idx = this.map.get(key);
    if (idx === undefined) return -1;

    const value = this.nodes[idx]!.value;
    this.moveToFront(idx);
    return value;
  }

  put(key: number, value: number): void {
    if (this.capacity <= 0) return;

    const existing = this.map.get(key);
    if (existing !== undefined) {
      this.nodes[existing]!.value = value;
      this.moveToFront(existing);
      return;
    }

    let idx: number;
    if (this.nodes.length < this.capacity) {
      this.nodes.push({ key, value, prev: -1, next: -1 });
      idx = this.nodes.length - 1;
    } else {
      // Full: reuse the least-recently-used node (the tail).
      idx = this.tail;
      const evictedKey = this.nodes[idx]!.key;
      this.map.delete(evictedKey);
      this.unlink(idx);
      this.nodes[idx]!.key = key;
      this.nodes[idx]!.value = value;
    }

    this.map.set(key, idx);
    this.linkFront(idx);
  }

  private unlink(idx: number): void {
    const node = this.nodes[idx]!;
    const prev = node.prev;
    const next = node.next;

    if (prev !== -1) {
      this.nodes[prev]!.next = next;
    } else {
      this.head = next;
    }
    if (next !== -1) {
      this.nodes[next]!.prev = prev;
    } else {
      this.tail = prev;
    }

    node.prev = -1;
    node.next = -1;
  }

  private linkFront(idx: number): void {
    const node = this.nodes[idx]!;
    node.prev = -1;
    node.next = this.head;

    if (this.head !== -1) {
      this.nodes[this.head]!.prev = idx;
    } else {
      this.tail = idx;
    }
    this.head = idx;
  }

  private moveToFront(idx: number): void {
    if (this.head === idx) return;
    this.unlink(idx);
    this.linkFront(idx);
  }
}

// Quick checks
const cache = new LRUCache(2);
cache.put(1, 1);
cache.put(2, 2);
console.log(cache.get(1)); // 1
cache.put(3, 3); // evicts key 2
console.log(cache.get(2)); // -1
cache.put(4, 4); // evicts key 1
console.log(cache.get(1)); // -1
console.log(cache.get(3)); // 3
console.log(cache.get(4)); // 4
```

### Rust

```rust
use std::collections::HashMap;

// A node in the doubly linked list. `prev`/`next` are indices into `nodes`,
// not pointers, which keeps the whole structure safe from borrow-checker pain.
#[derive(Clone, Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>, // key -> index into nodes
    nodes: Vec<Node>,         // storage; slots are never removed, only reused
    head: Option<usize>,      // most-recently-used node
    tail: Option<usize>,      // least-recently-used node
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = if capacity < 0 { 0 } else { capacity as usize };
        LRUCache {
            capacity,
            map: HashMap::new(),
            nodes: Vec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key).copied() {
            Some(idx) => {
                let value = self.nodes[idx].value;
                self.move_to_front(idx);
                value
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // Key already present: update its value and mark it most-recently-used.
        if let Some(idx) = self.map.get(&key).copied() {
            self.nodes[idx].value = value;
            self.move_to_front(idx);
            return;
        }

        // New key.
        let idx = if self.nodes.len() < self.capacity {
            self.nodes.push(Node::new(key, value));
            self.nodes.len() - 1
        } else {
            // Full: reuse the least-recently-used node (the tail).
            let idx = match self.tail {
                Some(t) => t,
                None => return, // cannot happen when capacity > 0
            };
            let evicted_key = self.nodes[idx].key;
            self.map.remove(&evicted_key);
            self.unlink(idx);
            self.nodes[idx].key = key;
            self.nodes[idx].value = value;
            idx
        };

        self.map.insert(key, idx);
        self.link_front(idx);
    }

    fn unlink(&mut self, idx: usize) {
        let (prev, next) = (self.nodes[idx].prev, self.nodes[idx].next);

        match prev {
            Some(p) => self.nodes[p].next = next,
            None => self.head = next,
        }
        match next {
            Some(n) => self.nodes[n].prev = prev,
            None => self.tail = prev,
        }

        self.nodes[idx].prev = None;
        self.nodes[idx].next = None;
    }

    fn link_front(&mut self, idx: usize) {
        self.nodes[idx].prev = None;
        self.nodes[idx].next = self.head;

        match self.head {
            Some(h) => self.nodes[h].prev = Some(idx),
            None => self.tail = Some(idx),
        }

        self.head = Some(idx);
    }

    fn move_to_front(&mut self, idx: usize) {
        if self.head == Some(idx) {
            return;
        }
        self.unlink(idx);
        self.link_front(idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn update_existing_value() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(1, 20); // update, still one entry
        assert_eq!(cache.get(1), 20);
        cache.put(2, 2);
        cache.put(3, 3); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn single_capacity() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(2, 2); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }
}
```

### Notes on the two implementations

- Both implementations keep the `Vec`/array **at most `capacity` long** by reusing the evicted tail slot instead of allocating, which is why the space claim is O(capacity) rather than O(number of puts).
- TypeScript uses a `-1` sentinel for "no neighbor"; Rust uses `Option<usize>`. They are the same idea, expressed with each language's idiom.
- The linked-list order (recency) lives entirely in the `prev`/`next` fields; the `Vec` order is arbitrary storage order. Moving a node never moves memory, only a few indices.

## Likely interviewer follow-ups

1. **LFU cache:** how would you add a frequency count and evict the least-frequently-used entry instead?
2. **Thread safety:** what would change to make `get`/`put` safe to call concurrently?
3. **Value-less variant:** how does the design simplify if you only need to track keys, not values?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Keep a `Vec` of nodes; each node stores `prev` and `next` as *indices* into that `Vec`.
2. Keep a `HashMap` from `key` to the node's index, and `head`/`tail` indices for the ends of the list.
3. On `get`, look up the index and move that node to the front.
4. On `put`, update-and-move if the key exists; otherwise insert a node (reusing the tail if full) and link it at the front.

## TypeScript, piece by piece

```typescript
interface LRUNode {
  key: number;
  value: number;
  prev: number; // index into nodes, -1 means "none"
  next: number;
}
```
A node stores its own `key` (so we can find and delete the key in the map when it is evicted) and its `value`. The `prev`/`next` links are array indices, with `-1` meaning "no neighbor".

```typescript
get(key: number): number {
  const idx = this.map.get(key);
  if (idx === undefined) return -1;
  const value = this.nodes[idx]!.value;
  this.moveToFront(idx);
  return value;
}
```
`Map.get` returns `number | undefined`; `undefined` means "not cached". `this.nodes[idx]!` asserts the index is valid — it is, because the map only ever holds indices that exist in `nodes`. Reading the value first, then moving, keeps the borrow simple.

```typescript
if (this.nodes.length < this.capacity) {
  this.nodes.push({ key, value, prev: -1, next: -1 });
  idx = this.nodes.length - 1;
} else {
  idx = this.tail; // reuse the LRU slot
  const evictedKey = this.nodes[idx]!.key;
  this.map.delete(evictedKey);
  this.unlink(idx);
  this.nodes[idx]!.key = key;
  this.nodes[idx]!.value = value;
}
```
When full, we do not push — we overwrite the tail node's slot with the new key/value after removing the old key from the map. This keeps the array from ever growing past `capacity`.

```typescript
private unlink(idx: number): void {
  const node = this.nodes[idx]!;
  const prev = node.prev;
  const next = node.next;
  if (prev !== -1) { this.nodes[prev]!.next = next; } else { this.head = next; }
  if (next !== -1) { this.nodes[next]!.prev = prev; } else { this.tail = prev; }
  node.prev = -1;
  node.next = -1;
}
```
To remove a node, we "splice it out": connect its previous neighbor to its next neighbor. If it had no previous neighbor it was the head, so the head moves forward; if no next neighbor it was the tail, so the tail moves back.

## Rust, piece by piece

```rust
struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}
```
This is the **Vec + indices** doubly linked list. Instead of `Box<Node>` pointers or `Rc<RefCell<Node>>`, each link is a `usize` index into the `Vec<Node>` owned by the cache. The `Vec` is the single owner of every node, so there are no borrow-checker fights, and "moving" a node means writing a couple of indices.

```rust
pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    head: Option<usize>,
    tail: Option<usize>,
}
```
`map` turns a key into an index in O(1); `nodes` stores the actual payloads; `head`/`tail` mark the list ends. `Option<usize>` means "there may be no such index yet", which is Rust's way of expressing the `-1` sentinel.

```rust
fn unlink(&mut self, idx: usize) {
    let (prev, next) = (self.nodes[idx].prev, self.nodes[idx].next);

    match prev {
        Some(p) => self.nodes[p].next = next,
        None => self.head = next,
    }
    match next {
        Some(n) => self.nodes[n].prev = prev,
        None => self.tail = prev,
    }

    self.nodes[idx].prev = None;
    self.nodes[idx].next = None;
}
```
We first *copy* `prev`/`next` out (both are `Copy`), which releases the borrow on `self.nodes[idx]` before we mutate the neighbors. Each `match` handles the "middle node" and "end node" cases explicitly.

```rust
fn move_to_front(&mut self, idx: usize) {
    if self.head == Some(idx) {
        return;
    }
    self.unlink(idx);
    self.link_front(idx);
}
```
`Some(idx)` is the "occupied" form of `Option<usize>`; comparing `self.head` to it tests whether `idx` is already the head. If it is, there is nothing to do — an important early-return that avoids unlinking and re-linking a node that is already first.

## Trace: capacity 2, classic example (should end with `get(4) → 4`)

| Step | Operation | Result | MRU → LRU (keys) |
|---|---|---|---|
| 1 | `put(1,1)` | — | `[1]` |
| 2 | `put(2,2)` | — | `[2,1]` |
| 3 | `get(1)` | `1` | `[1,2]` |
| 4 | `put(3,3)` | — | `[3,1]` (2 evicted) |
| 5 | `get(2)` | `-1` | `[3,1]` |
| 6 | `put(4,4)` | — | `[4,3]` (1 evicted) |
| 7 | `get(1)` | `-1` | `[4,3]` |
| 8 | `get(3)` | `3` | `[3,4]` |
| 9 | `get(4)` | `4` | `[4,3]` |

Each `get` moves the accessed key to the front; each `put` of a new key evicts the key at the tail.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| Node link | `prev: number` with `-1` | `prev: Option<usize>` with `None` |
| Key lookup | `Map<number, number>` | `HashMap<i32, usize>` |
| "Not found" | `undefined` | `None` |
| List end | `-1` sentinel | `None` |
| Mutate method | `this.nodes[idx]!.value = v` | `self.nodes[idx].value = v` |
| Optional field access | `map.get(key)` returns `number \| undefined` | `map.get(&key)` returns `Option<&usize>` |
