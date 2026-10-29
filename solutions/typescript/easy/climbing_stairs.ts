// Climbing Stairs — easy
// You can climb 1 or 2 steps at a time; return the number of distinct ways to climb n steps.

function climbStairs(n: number): number {
  if (n <= 2) return n;

  let first = 1;  // ways to climb 1 step
  let second = 2; // ways to climb 2 steps

  for (let i = 3; i <= n; i++) {
    const third = first + second;
    first = second;
    second = third;
  }

  return second;
}

// Quick checks
console.log(climbStairs(2)); // 2
console.log(climbStairs(3)); // 3
console.log(climbStairs(4)); // 5
console.log(climbStairs(5)); // 8
console.log(climbStairs(1)); // 1
