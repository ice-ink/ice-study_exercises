<?php

function isAnagram(string $s, string $t): bool
{
    return count_chars($s, 1) === count_chars($t, 1);
}

var_dump(isAnagram('anagram', 'nagaram')); // true
var_dump(isAnagram('rat', 'car')); // false
var_dump(isAnagram('a', 'ab')); // false
var_dump(isAnagram('', '')); // true
