// Reverse String — easy
// Reverse an array of characters in place; do not allocate extra space.

function reverseString(s: string[]): void {
  let left = 0;
  let right = s.length - 1;

  while (left < right) {
    const tmp = s[left]!;
    s[left] = s[right]!;
    s[right] = tmp;
    left++;
    right--;
  }
}

// Quick checks
const s1 = ["h", "e", "l", "l", "o"];
reverseString(s1);
console.log(s1); // ["o","l","l","e","h"]

const s2 = ["H", "a", "n", "n", "a", "h"];
reverseString(s2);
console.log(s2); // ["h","a","n","n","a","H"]

const s3 = ["a"];
reverseString(s3);
console.log(s3); // ["a"]

const s4: string[] = [];
reverseString(s4);
console.log(s4); // []
