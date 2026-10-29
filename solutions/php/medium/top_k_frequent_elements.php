<?php
// Top K Frequent Elements — medium
// Return the k most frequent elements (answer unique).

function topKFrequent(array $nums, int $k): array
{
    $freq = [];
    foreach ($nums as $num) {
        $freq[$num] = ($freq[$num] ?? 0) + 1;
    }

    $n = count($nums);
    $buckets = array_fill(0, $n + 1, []);
    foreach ($freq as $num => $count) {
        $buckets[$count][] = $num;
    }

    $result = [];
    for ($i = $n; $i >= 0 && count($result) < $k; $i--) {
        foreach ($buckets[$i] as $num) {
            $result[] = $num;
            if (count($result) === $k) {
                break;
            }
        }
    }

    return $result;
}

// Quick checks
var_dump(topKFrequent([1, 1, 1, 2, 2, 3], 2));           // [1, 2]
var_dump(topKFrequent([1], 1));                           // [1]
var_dump(topKFrequent([4, 4, 4, 2, 2, 2, 1, 1, 3], 3));   // [4, 2, 1]
