// Maximum Subarray — medium
// Return the largest sum of any contiguous subarray.

function maxSubArray(nums: number[]): number {
  let maxSoFar = nums[0]!;
  let current = 0;

  for (const n of nums) {
    if (current < 0) current = 0;
    current += n;
    if (current > maxSoFar) maxSoFar = current;
  }

  return maxSoFar;
}

// Quick checks
console.log(maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4])); // 6
console.log(maxSubArray([1]));                              // 1
console.log(maxSubArray([5, 4, -1, 7, 8]));                 // 23
console.log(maxSubArray([-2, -1]));                         // -1
