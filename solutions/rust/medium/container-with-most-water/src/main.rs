pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_water = 0;

    while left < right {
        let width = (right - left) as i32;
        let water = width * height[left].min(height[right]);
        if water > max_water {
            max_water = water;
        }

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_water
}

fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!("{}", max_area(vec![1, 1]));
    println!("{}", max_area(vec![4, 3, 2, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(max_area(vec![1, 2, 1]), 2);
    }
}
