// Remove Duplicates from Sorted Array — easy
// Given a sorted array, remove duplicates in place and return the new length (first k elements are unique).

function removeDuplicates(nums: number[]): number {
  if (nums.length === 0) return 0;

  let slow = 0;
  for (let fast = 1; fast < nums.length; fast++) {
    if (nums[fast] !== nums[slow]) {
      slow++;
      nums[slow] = nums[fast]!;
    }
  }

  return slow + 1;
}

// Quick checks
console.log(removeDuplicates([1, 1, 2])); // 2
console.log(removeDuplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4])); // 5
console.log(removeDuplicates([1])); // 1
console.log(removeDuplicates([]));  // 0
