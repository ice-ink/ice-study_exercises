<?php

// Minimum Window Substring — hard
// Return the minimum-length substring of s that contains all characters of t (with counts).

function minWindow(string $s, string $t): string
{
    if (strlen($s) < strlen($t)) {
        return "";
    }

    // Frequency map of the characters we must cover.
    $need = [];
    foreach (str_split($t) as $ch) {
        $need[$ch] = ($need[$ch] ?? 0) + 1;
    }

    $required = count($need); // distinct characters we must satisfy
    $window = [];
    $have = 0; // distinct characters currently satisfied
    $left = 0;
    $bestLeft = 0;
    $bestLen = PHP_INT_MAX;

    $n = strlen($s);
    for ($right = 0; $right < $n; $right++) {
        $ch = $s[$right];
        $window[$ch] = ($window[$ch] ?? 0) + 1;

        if (isset($need[$ch]) && $window[$ch] === $need[$ch]) {
            $have++;
        }

        // Shrink from the left while the window still satisfies all needs.
        while ($have === $required) {
            $len = $right - $left + 1;
            if ($len < $bestLen) {
                $bestLen = $len;
                $bestLeft = $left;
            }

            $leftCh = $s[$left];
            $window[$leftCh]--;
            if (isset($need[$leftCh]) && $window[$leftCh] < $need[$leftCh]) {
                $have--;
            }
            $left++;
        }
    }

    return $bestLen === PHP_INT_MAX ? "" : substr($s, $bestLeft, $bestLen);
}

// Quick checks
var_dump(minWindow("ADOBECODEBANC", "ABC")); // "BANC"
var_dump(minWindow("a", "a")); // "a"
var_dump(minWindow("a", "aa")); // ""
var_dump(minWindow("ab", "b")); // "b"
