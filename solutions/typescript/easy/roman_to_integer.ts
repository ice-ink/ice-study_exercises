// Roman to Integer — easy
// Convert a Roman numeral (I, V, X, L, C, D, M) to an integer.

function romanToInt(s: string): number {
  const values: Record<string, number> = {
    I: 1,
    V: 5,
    X: 10,
    L: 50,
    C: 100,
    D: 500,
    M: 1000,
  };

  let total = 0;
  for (let i = 0; i < s.length; i++) {
    const curr = values[s[i]!]!;
    const next = i + 1 < s.length ? values[s[i + 1]!]! : 0;
    total += curr < next ? -curr : curr;
  }

  return total;
}

// Quick checks
console.log(romanToInt("III"));     // 3
console.log(romanToInt("LVIII"));   // 58
console.log(romanToInt("MCMXCIV")); // 1994
console.log(romanToInt("IV"));      // 4
