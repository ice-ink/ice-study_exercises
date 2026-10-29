<?php
// Longest Substring Without Repeating Characters — medium
// Return the length of the longest substring with no repeating characters.

function lengthOfLongestSubstring(string $s): int
{
    $lastSeen = [];
    $left = 0;
    $maxLength = 0;

    for ($right = 0; $right < strlen($s); $right++) {
        $ch = $s[$right];
        if (isset($lastSeen[$ch]) && $lastSeen[$ch] >= $left) {
            $left = $lastSeen[$ch] + 1;
        }
        $lastSeen[$ch] = $right;
        $maxLength = max($maxLength, $right - $left + 1);
    }

    return $maxLength;
}

// Quick checks
var_dump(lengthOfLongestSubstring("abcabcbb")); // 3
var_dump(lengthOfLongestSubstring("bbbbb"));    // 1
var_dump(lengthOfLongestSubstring("pwwkew"));   // 3
var_dump(lengthOfLongestSubstring(""));         // 0
var_dump(lengthOfLongestSubstring("aab"));      // 2
