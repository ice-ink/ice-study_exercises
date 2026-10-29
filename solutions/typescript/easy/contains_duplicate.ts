// Contains Duplicate — easy
// Return true if any value appears at least twice in the array.

function containsDuplicate(nums: number[]): boolean {
  const seen = new Set<number>();

  for (const n of nums) {
    if (seen.has(n)) return true;
    seen.add(n);
  }

  return false;
}

// Quick checks
console.log(containsDuplicate([1, 2, 3, 1])); // true
console.log(containsDuplicate([1, 2, 3, 4])); // false
console.log(containsDuplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])); // true
console.log(containsDuplicate([])); // false
