// Container With Most Water — medium
// Given bar heights, return the max water a container between two bars can hold.

function maxArea(height: number[]): number {
  let left = 0;
  let right = height.length - 1;
  let maxWater = 0;

  while (left < right) {
    const width = right - left;
    const water = width * Math.min(height[left]!, height[right]!);
    if (water > maxWater) maxWater = water;

    if (height[left]! < height[right]!) {
      left++;
    } else {
      right--;
    }
  }

  return maxWater;
}

// Quick checks
console.log(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
console.log(maxArea([1, 1]));                      // 1
console.log(maxArea([4, 3, 2, 1, 4]));             // 16
console.log(maxArea([1, 2, 1]));                   // 2
