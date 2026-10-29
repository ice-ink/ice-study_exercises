<?php

function findMedianSortedArrays(array $nums1, array $nums2): float
{
    if (count($nums1) > count($nums2)) {
        return findMedianSortedArrays($nums2, $nums1);
    }

    $m = count($nums1);
    $n = count($nums2);
    $total = $m + $n;
    $half = intdiv($total + 1, 2);

    $low = 0;
    $high = $m;

    while ($low <= $high) {
        $i = intdiv($low + $high, 2);
        $j = $half - $i;

        $left1 = $i === 0 ? PHP_INT_MIN : $nums1[$i - 1];
        $right1 = $i === $m ? PHP_INT_MAX : $nums1[$i];
        $left2 = $j === 0 ? PHP_INT_MIN : $nums2[$j - 1];
        $right2 = $j === $n ? PHP_INT_MAX : $nums2[$j];

        if ($left1 <= $right2 && $left2 <= $right1) {
            if ($total % 2 === 1) {
                return (float)max($left1, $left2);
            }
            return (max($left1, $left2) + min($right1, $right2)) / 2.0;
        } elseif ($left1 > $right2) {
            $high = $i - 1;
        } else {
            $low = $i + 1;
        }
    }

    throw new RuntimeException('input arrays are not sorted');
}

var_dump(findMedianSortedArrays([1, 3], [2])); // 2
var_dump(findMedianSortedArrays([1, 2], [3, 4])); // 2.5
var_dump(findMedianSortedArrays([0, 0], [0, 0])); // 0
var_dump(findMedianSortedArrays([], [1])); // 1
var_dump(findMedianSortedArrays([2], [])); // 2
