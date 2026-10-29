// Single Number — easy
// Every element appears twice except one; return that one.

function singleNumber(nums: number[]): number {
  let result = 0;

  for (const n of nums) {
    result ^= n;
  }

  return result;
}

// Quick checks
console.log(singleNumber([2, 2, 1])); // 1
console.log(singleNumber([4, 1, 2, 1, 2])); // 4
console.log(singleNumber([1])); // 1
