pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut results: Vec<Vec<String>> = Vec::new();
    let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n]; // row - col + (n - 1), always in 0..2n
    let mut diag2 = vec![false; 2 * n]; // row + col, always in 0..2n

    fn backtrack(
        row: usize,
        n: usize,
        board: &mut Vec<Vec<char>>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        results: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            let solution: Vec<String> = board
                .iter()
                .map(|r| r.iter().collect::<String>())
                .collect();
            results.push(solution);
            return;
        }

        for col in 0..n {
            let d1 = row + n - 1 - col;
            let d2 = row + col;
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }

            board[row][col] = 'Q';
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;

            backtrack(row + 1, n, board, cols, diag1, diag2, results);

            board[row][col] = '.';
            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }

    backtrack(0, n, &mut board, &mut cols, &mut diag1, &mut diag2, &mut results);
    results
}

fn main() {
    println!("{:?}", solve_n_queens(4));
    println!("{:?}", solve_n_queens(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let solutions = solve_n_queens(4);
        assert_eq!(solutions.len(), 2);
        assert_eq!(
            solutions,
            vec![
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ],
            ]
        );
    }

    #[test]
    fn no_solution_for_small_boards() {
        assert!(solve_n_queens(2).is_empty());
        assert!(solve_n_queens(3).is_empty());
    }

    #[test]
    fn single_cell() {
        assert_eq!(solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }
}
