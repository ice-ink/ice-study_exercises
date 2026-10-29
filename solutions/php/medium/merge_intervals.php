<?php

function merge(array $intervals): array
{
    usort($intervals, fn (array $a, array $b) => $a[0] <=> $b[0]);

    $merged = [];
    foreach ($intervals as $interval) {
        $last = count($merged) - 1;
        if ($last < 0 || $interval[0] > $merged[$last][1]) {
            $merged[] = $interval;
        } else {
            $merged[$last][1] = max($merged[$last][1], $interval[1]);
        }
    }

    return $merged;
}

var_dump(merge([[1, 3], [2, 6], [8, 10], [15, 18]])); // [[1,6],[8,10],[15,18]]
var_dump(merge([[1, 4], [4, 5]]));                    // [[1,5]]
var_dump(merge([[1, 4], [0, 4]]));                    // [[0,4]]
var_dump(merge([[1, 4], [5, 6]]));                    // [[1,4],[5,6]]
var_dump(merge([]));                                  // []
