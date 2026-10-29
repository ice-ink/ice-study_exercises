// Merge Intervals — medium
// Merge all overlapping intervals [start, end] into disjoint intervals.

function merge(intervals: number[][]): number[][] {
  const sorted = [...intervals].sort((a, b) => a[0]! - b[0]!);
  const merged: number[][] = [];

  for (const interval of sorted) {
    const last = merged[merged.length - 1];
    if (last === undefined || interval[0]! > last[1]!) {
      merged.push(interval);
    } else {
      last[1] = Math.max(last[1]!, interval[1]!);
    }
  }

  return merged;
}

// Quick checks
console.log(JSON.stringify(merge([[1, 3], [2, 6], [8, 10], [15, 18]]))); // [[1,6],[8,10],[15,18]]
console.log(JSON.stringify(merge([[1, 4], [4, 5]])));                    // [[1,5]]
console.log(JSON.stringify(merge([[1, 4], [0, 4]])));                    // [[0,4]]
console.log(JSON.stringify(merge([[1, 4], [5, 6]])));                    // [[1,4],[5,6]]
console.log(JSON.stringify(merge([])));                                  // []
