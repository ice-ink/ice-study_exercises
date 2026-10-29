def solve_n_queens(n: int) -> list[list[str]]:
    results: list[list[str]] = []
    board: list[list[str]] = [["."] * n for _ in range(n)]

    cols: set[int] = set()  # used columns
    diag1: set[int] = set()  # row - col (anti-diagonal)
    diag2: set[int] = set()  # row + col (main diagonal)

    def backtrack(row: int) -> None:
        if row == n:
            results.append(["".join(r) for r in board])
            return

        for col in range(n):
            d1 = row - col
            d2 = row + col
            if col in cols or d1 in diag1 or d2 in diag2:
                continue

            board[row][col] = "Q"
            cols.add(col)
            diag1.add(d1)
            diag2.add(d2)

            backtrack(row + 1)

            board[row][col] = "."
            cols.remove(col)
            diag1.remove(d1)
            diag2.remove(d2)

    backtrack(0)
    return results


print(solve_n_queens(4));  # [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
print(solve_n_queens(1));  # [["Q"]]
print(solve_n_queens(2));  # []
print(solve_n_queens(3));  # []
