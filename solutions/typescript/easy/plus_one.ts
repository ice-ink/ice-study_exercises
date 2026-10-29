// Plus One — easy
// Given the digits of a non-negative integer (most significant first), increment by one and return the resulting digits.

function plusOne(digits: number[]): number[] {
  for (let i = digits.length - 1; i >= 0; i--) {
    const digit = digits[i]!;
    if (digit < 9) {
      digits[i] = digit + 1;
      return digits;
    }
    digits[i] = 0;
  }

  return [1, ...digits];
}

// Quick checks
console.log(plusOne([1, 2, 3])); // [1, 2, 4]
console.log(plusOne([4, 3, 2, 1])); // [4, 3, 2, 2]
console.log(plusOne([9])); // [1, 0]
console.log(plusOne([9, 9, 9])); // [1, 0, 0, 0]
