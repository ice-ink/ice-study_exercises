<?php

function isPalindrome(string $s): bool
{
    $left = 0;
    $right = strlen($s) - 1;

    while ($left < $right) {
        $l = $s[$left];
        $r = $s[$right];

        if (!ctype_alnum($l)) {
            $left++;
            continue;
        }
        if (!ctype_alnum($r)) {
            $right--;
            continue;
        }
        if (strtolower($l) !== strtolower($r)) {
            return false;
        }
        $left++;
        $right--;
    }

    return true;
}

var_dump(isPalindrome('A man, a plan, a canal: Panama')); // true
var_dump(isPalindrome('hello'));                          // false
var_dump(isPalindrome(' '));                              // true
