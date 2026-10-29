// Best Time to Buy and Sell Stock — easy
// Choose one day to buy and one later day to sell to maximize profit; return 0 if no profit is possible.

function maxProfit(prices: number[]): number {
  let minPrice = Infinity;
  let best = 0;

  for (const price of prices) {
    minPrice = Math.min(minPrice, price);
    best = Math.max(best, price - minPrice);
  }

  return best;
}

// Quick checks
console.log(maxProfit([7, 1, 5, 3, 6, 4])); // 5
console.log(maxProfit([7, 6, 4, 3, 1]));    // 0
console.log(maxProfit([1, 2]));             // 1
console.log(maxProfit([1]));                // 0
