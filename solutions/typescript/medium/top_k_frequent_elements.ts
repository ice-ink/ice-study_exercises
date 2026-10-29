// Top K Frequent Elements — medium
// Return the k most frequent elements (answer unique).

function topKFrequent(nums: number[], k: number): number[] {
  const freq = new Map<number, number>();
  for (const num of nums) {
    freq.set(num, (freq.get(num) ?? 0) + 1);
  }

  // buckets[i] holds every number that appears exactly i times.
  const buckets: number[][] = Array.from({ length: nums.length + 1 }, () => []);
  for (const [num, count] of freq) {
    buckets[count]!.push(num);
  }

  const result: number[] = [];
  for (let i = buckets.length - 1; i >= 0 && result.length < k; i--) {
    const bucket = buckets[i]!;
    for (const num of bucket) {
      result.push(num);
      if (result.length === k) break;
    }
  }

  return result;
}

// Quick checks
console.log(topKFrequent([1, 1, 1, 2, 2, 3], 2));        // [1, 2]
console.log(topKFrequent([1], 1));                        // [1]
console.log(topKFrequent([4, 4, 4, 2, 2, 2, 1, 1, 3], 3)); // [4, 2, 1]
