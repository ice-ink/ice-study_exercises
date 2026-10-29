<?php

function firstUniqChar(string $s): int
{
    $counts = count_chars($s, 1); // byte value -> frequency

    for ($i = 0, $len = strlen($s); $i < $len; $i++) {
        if ($counts[ord($s[$i])] === 1) {
            return $i;
        }
    }

    return -1;
}

var_dump(firstUniqChar('leetcode')); // 0
var_dump(firstUniqChar('loveleetcode')); // 2
var_dump(firstUniqChar('aabb')); // -1
var_dump(firstUniqChar('')); // -1
