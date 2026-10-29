<?php

function longestValidParentheses(string $s): int
{
    $stack = [-1];
    $maxLen = 0;

    for ($i = 0, $len = strlen($s); $i < $len; $i++) {
        if ($s[$i] === '(') {
            $stack[] = $i;
        } else {
            array_pop($stack);
            if ($stack === []) {
                $stack[] = $i;
            } else {
                $maxLen = max($maxLen, $i - $stack[count($stack) - 1]);
            }
        }
    }

    return $maxLen;
}

var_dump(longestValidParentheses("(()")); // 2
var_dump(longestValidParentheses(")()())")); // 4
var_dump(longestValidParentheses("")); // 0
var_dump(longestValidParentheses("()(()")); // 2
var_dump(longestValidParentheses("()()")); // 4
