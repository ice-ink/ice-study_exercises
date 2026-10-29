<?php

function containsDuplicate(array $nums): bool
{
    $seen = [];

    foreach ($nums as $n) {
        if (isset($seen[$n])) {
            return true;
        }
        $seen[$n] = true;
    }

    return false;
}

var_dump(containsDuplicate([1, 2, 3, 1])); // true
var_dump(containsDuplicate([1, 2, 3, 4])); // false
var_dump(containsDuplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])); // true
var_dump(containsDuplicate([])); // false
