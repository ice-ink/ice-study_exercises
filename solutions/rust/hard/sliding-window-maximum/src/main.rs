use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if k == 0 || k > n {
        return Vec::new();
    }

    let mut result: Vec<i32> = Vec::with_capacity(n - k + 1);
    // deque holds indices; the values at those indices are strictly decreasing.
    let mut deque: VecDeque<usize> = VecDeque::new();

    for (i, &value) in nums.iter().enumerate() {
        // Drop indices that have slid out of the window's left edge.
        if let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            }
        }

        // Pop from the back while the incoming value is >= the back's value.
        while let Some(&back) = deque.back() {
            if nums[back] <= value {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        // Once the first window is full, the front of the deque is the maximum.
        if i + 1 >= k {
            result.push(nums[deque[0]]);
        }
    }

    result
}

fn main() {
    println!("{:?}", max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
    println!("{:?}", max_sliding_window(vec![1], 1));
    println!("{:?}", max_sliding_window(vec![7, 2, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
        assert_eq!(max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
    }

    #[test]
    fn empty_when_k_too_large() {
        assert!(max_sliding_window(vec![1, 2], 3).is_empty());
    }
}
