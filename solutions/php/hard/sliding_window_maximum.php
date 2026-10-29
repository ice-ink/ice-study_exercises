<?php

function maxSlidingWindow(array $nums, int $k): array
{
    $n = count($nums);
    $result = [];
    // deque holds indices; the values at those indices are strictly decreasing.
    $deque = [];

    for ($i = 0; $i < $n; $i++) {
        // Drop indices that have slid out of the window's left edge.
        if (count($deque) > 0 && $deque[0] < $i - $k + 1) {
            array_shift($deque);
        }

        // Pop from the back while the incoming value is >= the back's value.
        while (count($deque) > 0 && $nums[$deque[count($deque) - 1]] <= $nums[$i]) {
            array_pop($deque);
        }

        $deque[] = $i;

        // Once the first window is full, the front of the deque is the maximum.
        if ($i >= $k - 1) {
            $result[] = $nums[$deque[0]];
        }
    }

    return $result;
}

var_dump(maxSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3)); // [3,3,5,5,6,7]
var_dump(maxSlidingWindow([1], 1)); // [1]
var_dump(maxSlidingWindow([1, -1], 1)); // [1,-1]
var_dump(maxSlidingWindow([7, 2, 4], 2)); // [7,4]
