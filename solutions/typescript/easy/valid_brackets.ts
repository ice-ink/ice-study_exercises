// Valid Brackets — easy
// Given a string of (), [], {} brackets, return whether it is balanced and correctly ordered.

function isValid(s: string): boolean {
  const closeToOpen: Record<string, string> = {
    ")": "(",
    "]": "[",
    "}": "{",
  };
  const stack: string[] = [];

  for (const ch of s) {
    if (ch in closeToOpen) {
      // pop() returns undefined on an empty stack, which never matches
      if (stack.pop() !== closeToOpen[ch]) return false;
    } else {
      stack.push(ch);
    }
  }

  return stack.length === 0;
}

// Quick checks
console.log(isValid("()[]{}")); // true
console.log(isValid("([)]"));   // false
console.log(isValid("(("));     // false
console.log(isValid(""));       // true
