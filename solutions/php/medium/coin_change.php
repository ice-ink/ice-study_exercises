<?php

function coinChange(array $coins, int $amount): int
{
    $dp = array_fill(0, $amount + 1, $amount + 1);
    $dp[0] = 0;

    for ($a = 1; $a <= $amount; $a++) {
        foreach ($coins as $coin) {
            if ($coin <= $a) {
                $dp[$a] = min($dp[$a], $dp[$a - $coin] + 1);
            }
        }
    }

    return $dp[$amount] > $amount ? -1 : $dp[$amount];
}

var_dump(coinChange([1, 2, 5], 11)); // 3  (5+5+1)
var_dump(coinChange([2], 3)); // -1
var_dump(coinChange([1], 0)); // 0
var_dump(coinChange([2, 5, 10, 1], 27)); // 4  (10+10+5+2)
var_dump(coinChange([186, 419, 83, 408], 6249)); // 20
