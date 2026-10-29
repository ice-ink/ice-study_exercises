// Valid Palindrome — easy
// Given a string, return true if it reads the same forwards and backwards,
// ignoring case and any character that isn't a letter or digit.

function isPalindrome(s: string): boolean {
  // Regex test: is this character a letter or digit? (the i makes it case-insensitive)
  const isAlnum = (c: string): boolean => /[a-z0-9]/i.test(c);

  let left = 0;
  let right = s.length - 1;

  while (left < right) {
    if (!isAlnum(s[left]!)) {
      left++;
      continue;
    }
    if (!isAlnum(s[right]!)) {
      right--;
      continue;
    }
    if (s[left]!.toLowerCase() !== s[right]!.toLowerCase()) {
      return false;
    }
    left++;
    right--;
  }

  return true;
}

// Quick checks
console.log(isPalindrome("A man, a plan, a canal: Panama")); // true
console.log(isPalindrome("hello"));                          // false
console.log(isPalindrome(" "));                              // true
