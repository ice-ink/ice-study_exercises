// Rotate Array — medium
// Rotate the array to the right by k steps, in place.

function rotate(nums: number[], k: number): void {
  const n = nums.length;
  if (n === 0) return;
  const steps = k % n;

  reverse(nums, 0, n - 1);
  reverse(nums, 0, steps - 1);
  reverse(nums, steps, n - 1);
}

function reverse(nums: number[], start: number, end: number): void {
  while (start < end) {
    const tmp = nums[start]!;
    nums[start] = nums[end]!;
    nums[end] = tmp;
    start++;
    end--;
  }
}

// Quick checks
const a1 = [1, 2, 3, 4, 5, 6, 7];
rotate(a1, 3);
console.log(a1); // [5, 6, 7, 1, 2, 3, 4]

const a2 = [-1, -100, 3, 99];
rotate(a2, 2);
console.log(a2); // [3, 99, -1, -100]

const a3 = [1, 2, 3];
rotate(a3, 4);
console.log(a3); // [3, 1, 2]

const a4 = [1, 2, 3, 4];
rotate(a4, 0);
console.log(a4); // [1, 2, 3, 4]
