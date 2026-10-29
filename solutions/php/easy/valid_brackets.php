<?php

function isValid(string $s): bool
{
    $closeToOpen = [
        ')' => '(',
        ']' => '[',
        '}' => '{',
    ];

    $stack = [];

    foreach (str_split($s) as $ch) {
        if (isset($closeToOpen[$ch])) {
            if (array_pop($stack) !== $closeToOpen[$ch]) {
                return false;
            }
        } else {
            $stack[] = $ch;
        }
    }

    return $stack === [];
}

var_dump(isValid('()[]{}')); // true
var_dump(isValid('([)]'));   // false
var_dump(isValid('(('));     // false
var_dump(isValid(''));       // true
