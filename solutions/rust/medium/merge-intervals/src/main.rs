pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by_key(|iv| iv[0]);
    let mut merged: Vec<Vec<i32>> = Vec::new();

    for interval in intervals {
        match merged.last_mut() {
            Some(last) if interval[0] <= last[1] => {
                last[1] = last[1].max(interval[1]);
            }
            _ => merged.push(interval),
        }
    }

    merged
}

fn main() {
    println!("{:?}", merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
    println!("{:?}", merge(vec![vec![1, 4], vec![4, 5]]));
    println!("{:?}", merge(vec![vec![1, 4], vec![0, 4]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
        assert_eq!(merge(vec![vec![1, 4], vec![0, 4]]), vec![vec![0, 4]]);
        assert_eq!(merge(vec![vec![1, 4], vec![5, 6]]), vec![vec![1, 4], vec![5, 6]]);
        assert_eq!(merge(vec![]), Vec::<Vec<i32>>::new());
    }
}
