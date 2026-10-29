pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0usize;
    let mut right = height.len();
    let mut left_max = 0;
    let mut right_max = 0;
    let mut water = 0;

    while left < right {
        if height[left] < height[right - 1] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right - 1] >= right_max {
                right_max = height[right - 1];
            } else {
                water += right_max - height[right - 1];
            }
            right -= 1;
        }
    }

    water
}

fn main() {
    println!("{:?}", trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(trap(vec![1, 2, 3]), 0);
        assert_eq!(trap(vec![3, 2, 1]), 0);
        assert_eq!(trap(vec![]), 0);
    }
}
