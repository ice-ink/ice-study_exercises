<?php

function reverseString(array &$s): void
{
    $left = 0;
    $right = count($s) - 1;

    while ($left < $right) {
        $tmp = $s[$left];
        $s[$left] = $s[$right];
        $s[$right] = $tmp;
        $left++;
        $right--;
    }
}

$s1 = ['h', 'e', 'l', 'l', 'o'];
reverseString($s1);
var_dump($s1); // ['o', 'l', 'l', 'e', 'h']

$s2 = ['H', 'a', 'n', 'n', 'a', 'h'];
reverseString($s2);
var_dump($s2); // ['h', 'a', 'n', 'n', 'a', 'H']

$s3 = ['a'];
reverseString($s3);
var_dump($s3); // ['a']

$s4 = [];
reverseString($s4);
var_dump($s4); // []
