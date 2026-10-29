pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let m = a.len();
    let n = b.len();
    let total = m + n;
    let half = (total + 1) / 2;

    let mut low = 0usize;
    let mut high = m;

    while low <= high {
        let i = (low + high) / 2;
        let j = half - i;

        let left1 = if i == 0 { i32::MIN } else { a[i - 1] };
        let right1 = if i == m { i32::MAX } else { a[i] };
        let left2 = if j == 0 { i32::MIN } else { b[j - 1] };
        let right2 = if j == n { i32::MAX } else { b[j] };

        if left1 <= right2 && left2 <= right1 {
            return if total % 2 == 1 {
                left1.max(left2) as f64
            } else {
                (left1.max(left2) + right1.min(right2)) as f64 / 2.0
            };
        } else if left1 > right2 {
            high = i - 1;
        } else {
            low = i + 1;
        }
    }

    unreachable!("input arrays must be sorted")
}

fn main() {
    println!("{:?}", find_median_sorted_arrays(vec![1, 3], vec![2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2.0);
    }
}
