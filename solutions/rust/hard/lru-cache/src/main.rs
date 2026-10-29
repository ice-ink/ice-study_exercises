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

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1)); // 1
    cache.put(3, 3); // evicts key 2
    println!("{}", cache.get(2)); // -1
    cache.put(4, 4); // evicts key 1
    println!("{}", cache.get(1)); // -1
    println!("{}", cache.get(3)); // 3
    println!("{}", cache.get(4)); // 4
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
