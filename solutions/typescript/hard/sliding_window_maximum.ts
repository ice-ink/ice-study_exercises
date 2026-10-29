// Sliding Window Maximum — hard
// Return the maximum of each sliding window of size k as it moves left to right.

function maxSlidingWindow(nums: number[], k: number): number[] {
  const n = nums.length;
  const result: number[] = new Array<number>(n - k + 1);
  // deque holds indices; the values at those indices are strictly decreasing.
  const deque: number[] = [];

  for (let i = 0; i < n; i++) {
    // Drop indices that have slid out of the window's left edge.
    if (deque.length > 0 && deque[0]! < i - k + 1) {
      deque.shift();
    }

    // Pop from the back while the incoming value is >= the back's value.
    while (deque.length > 0 && nums[deque[deque.length - 1]!]! <= nums[i]!) {
      deque.pop();
    }

    deque.push(i);

    // Once the first window is full, the front of the deque is the maximum.
    if (i >= k - 1) {
      result[i - k + 1] = nums[deque[0]!]!;
    }
  }

  return result;
}

// Quick checks
console.log(maxSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3)); // [3,3,5,5,6,7]
console.log(maxSlidingWindow([1], 1)); // [1]
console.log(maxSlidingWindow([1, -1], 1)); // [1,-1]
console.log(maxSlidingWindow([7, 2, 4], 2)); // [7,4]
