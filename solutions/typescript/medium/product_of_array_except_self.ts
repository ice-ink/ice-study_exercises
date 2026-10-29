// Product of Array Except Self — medium
// Return output[i] = product of all elements except nums[i], without division.

function productExceptSelf(nums: number[]): number[] {
  const n = nums.length;
  const result: number[] = new Array<number>(n).fill(1);

  let prefix = 1;
  for (let i = 0; i < n; i++) {
    result[i] = prefix;
    prefix *= nums[i]!;
  }

  let suffix = 1;
  for (let i = n - 1; i >= 0; i--) {
    result[i] = result[i]! * suffix;
    suffix *= nums[i]!;
  }

  return result;
}

// Quick checks
console.log(productExceptSelf([1, 2, 3, 4]));      // [24, 12, 8, 6]
console.log(productExceptSelf([-1, 1, 0, -3, 3])); // [0, 0, 9, 0, 0]
console.log(productExceptSelf([0, 0]));            // [0, 0]
console.log(productExceptSelf([2, 3]));            // [3, 2]
