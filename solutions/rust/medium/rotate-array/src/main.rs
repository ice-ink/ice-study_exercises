pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    if n == 0 {
        return;
    }
    let steps = (k % n as i32) as usize;

    nums.reverse();
    nums[..steps].reverse();
    nums[steps..].reverse();
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut a, 3);
    println!("{:?}", a);

    let mut b = vec![-1, -100, 3, 99];
    rotate(&mut b, 2);
    println!("{:?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut a, 3);
        assert_eq!(a, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut b = vec![-1, -100, 3, 99];
        rotate(&mut b, 2);
        assert_eq!(b, vec![3, 99, -1, -100]);

        let mut c = vec![1, 2, 3];
        rotate(&mut c, 4);
        assert_eq!(c, vec![3, 1, 2]);

        let mut d = vec![1, 2, 3, 4];
        rotate(&mut d, 0);
        assert_eq!(d, vec![1, 2, 3, 4]);
    }
}
