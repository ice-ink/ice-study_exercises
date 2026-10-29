pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut grid = grid;
    let mut islands = 0;

    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || grid[r][c] != '1' {
            return;
        }
        grid[r][c] = '0';
        dfs(grid, r + 1, c, rows, cols);
        if r > 0 {
            dfs(grid, r - 1, c, rows, cols);
        }
        dfs(grid, r, c + 1, rows, cols);
        if c > 0 {
            dfs(grid, r, c - 1, rows, cols);
        }
    }

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                islands += 1;
                dfs(&mut grid, r, c, rows, cols);
            }
        }
    }

    islands
}

fn main() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    println!("{:?}", num_islands(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ]),
            3
        );
        assert_eq!(num_islands(vec![vec!['1']]), 1);
        assert_eq!(num_islands(vec![vec!['0']]), 0);
        assert_eq!(
            num_islands(vec![
                vec!['1', '0', '1'],
                vec!['0', '1', '0'],
                vec!['1', '0', '1'],
            ]),
            5
        );
    }
}
