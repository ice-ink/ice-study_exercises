<?php
// Longest Palindromic Substring — medium
// Return the longest palindromic substring in s.

function longestPalindrome(string $s): string
{
    $n = strlen($s);
    $start = 0;
    $maxLen = 0;

    $expandAroundCenter = function (int $left, int $right) use ($s, $n, &$start, &$maxLen): void {
        while ($left >= 0 && $right < $n && $s[$left] === $s[$right]) {
            $len = $right - $left + 1;
            if ($len > $maxLen) {
                $maxLen = $len;
                $start = $left;
            }
            $left--;
            $right++;
        }
    };

    for ($i = 0; $i < $n; $i++) {
        $expandAroundCenter($i, $i);     // odd-length palindromes
        $expandAroundCenter($i, $i + 1); // even-length palindromes
    }

    return substr($s, $start, $maxLen);
}

// Quick checks
var_dump(longestPalindrome("babad"));   // "bab"
var_dump(longestPalindrome("cbbd"));    // "bb"
var_dump(longestPalindrome("racecar")); // "racecar"
var_dump(longestPalindrome("a"));       // "a"
var_dump(longestPalindrome(""));        // ""
