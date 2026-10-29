// Two Sum — easy
// Given an array of integers and a target, return the indices of the two numbers that add up to target.

function twoSum(nums: number[], target: number): number[] {
  const seen = new Map<number, number>(); // value -> index

  for (const [i, num] of nums.entries()) {
    const complement = target - num;
    const found = seen.get(complement);
    if (found !== undefined) {
      return [found, i];
    }
    seen.set(num, i);
  }

  return []; // unreachable: exactly one solution is guaranteed
}

// Quick checks
console.log(twoSum([2, 7, 11, 15], 9)); // [0, 1]
console.log(twoSum([3, 2, 4], 6));      // [1, 2]
console.log(twoSum([3, 3], 6));         // [0, 1]
