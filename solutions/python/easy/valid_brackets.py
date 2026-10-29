def is_valid(s: str) -> bool:
    pairs = {")": "(", "]": "[", "}": "{"}
    stack: list[str] = []
    for ch in s:
        if ch in "([{":
            stack.append(ch)
        elif not stack or stack.pop() != pairs[ch]:
            return False
    return not stack


print(is_valid("()"));  # True
print(is_valid("()[]{}"));  # True
print(is_valid("(]"));  # False
print(is_valid("([)]"));  # False
print(is_valid("{[]}"));  # True
