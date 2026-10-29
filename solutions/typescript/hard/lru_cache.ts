// LRU Cache — hard
// Implement a least-recently-used cache with O(1) get and put.

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
