// Trapping Rain Water — hard
// Return how much water can be trapped between the bars after raining.

function trap(height: number[]): number {
  let left = 0;
  let right = height.length - 1;
  let leftMax = 0;
  let rightMax = 0;
  let water = 0;

  while (left < right) {
    const leftHeight = height[left]!;
    const rightHeight = height[right]!;
    if (leftHeight < rightHeight) {
      if (leftHeight >= leftMax) {
        leftMax = leftHeight;
      } else {
        water += leftMax - leftHeight;
      }
      left++;
    } else {
      if (rightHeight >= rightMax) {
        rightMax = rightHeight;
      } else {
        water += rightMax - rightHeight;
      }
      right--;
    }
  }

  return water;
}

// Quick checks
console.log(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
console.log(trap([4, 2, 0, 3, 2, 5])); // 9
console.log(trap([1, 2, 3])); // 0
console.log(trap([3, 2, 1])); // 0
console.log(trap([])); // 0
