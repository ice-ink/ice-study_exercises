// Longest Valid Parentheses — hard
// Return the length of the longest well-formed parentheses substring.

function longestValidParentheses(s: string): number {
  const stack: number[] = [-1];
  let maxLen = 0;

  for (let i = 0; i < s.length; i++) {
    if (s[i] === "(") {
      stack.push(i);
    } else {
      stack.pop();
      if (stack.length === 0) {
        stack.push(i);
      } else {
        maxLen = Math.max(maxLen, i - stack[stack.length - 1]!);
      }
    }
  }

  return maxLen;
}

// Quick checks
console.log(longestValidParentheses("(()")); // 2
console.log(longestValidParentheses(")()())")); // 4
console.log(longestValidParentheses("")); // 0
console.log(longestValidParentheses("()(()")); // 2
console.log(longestValidParentheses("()()")); // 4
