// Coin Change — medium
// Return the fewest number of coins that make up the amount, or -1 if it can't be made.

function coinChange(coins: number[], amount: number): number {
  const dp = new Array<number>(amount + 1).fill(Infinity);
  dp[0] = 0;

  for (let a = 1; a <= amount; a++) {
    for (const coin of coins) {
      if (coin <= a) {
        dp[a] = Math.min(dp[a]!, dp[a - coin]! + 1);
      }
    }
  }

  return dp[amount]! === Infinity ? -1 : dp[amount]!;
}

// Quick checks
console.log(coinChange([1, 2, 5], 11)); // 3  (5+5+1)
console.log(coinChange([2], 3)); // -1
console.log(coinChange([1], 0)); // 0
console.log(coinChange([2, 5, 10, 1], 27)); // 4  (10+10+5+2)
console.log(coinChange([186, 419, 83, 408], 6249)); // 20
