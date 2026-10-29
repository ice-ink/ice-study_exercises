<?php

// LRU Cache — hard
// Least-recently-used cache using an ordered associative array: insertion order
// tracks recency, so the front (array_key_first) is always the LRU entry.

class LRUCache
{
    private array $cache = [];

    public function __construct(private int $capacity)
    {
    }

    public function get(int $key): int
    {
        if (!array_key_exists($key, $this->cache)) {
            return -1;
        }

        $value = $this->cache[$key];
        // Move to the most-recently-used end by unsetting and re-adding.
        unset($this->cache[$key]);
        $this->cache[$key] = $value;

        return $value;
    }

    public function put(int $key, int $value): void
    {
        if ($this->capacity <= 0) {
            return;
        }

        if (array_key_exists($key, $this->cache)) {
            unset($this->cache[$key]);
        }

        $this->cache[$key] = $value;

        // Evict the least-recently-used entries while over capacity.
        while (count($this->cache) > $this->capacity) {
            $lruKey = array_key_first($this->cache);
            unset($this->cache[$lruKey]);
        }
    }
}

// Quick checks
$cache = new LRUCache(2);
$cache->put(1, 1);
$cache->put(2, 2);
var_dump($cache->get(1)); // int(1)
$cache->put(3, 3); // evicts key 2
var_dump($cache->get(2)); // int(-1)
$cache->put(4, 4); // evicts key 1
var_dump($cache->get(1)); // int(-1)
var_dump($cache->get(3)); // int(3)
var_dump($cache->get(4)); // int(4)
