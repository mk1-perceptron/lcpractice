fn main() {
    println!("Hello, world!");
}

fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, val| acc ^ val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let nums = vec![2, 2, 1];
        let result = single_number(nums);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_two() {
        let nums = vec![4, 1, 2, 1, 2];
        let result = single_number(nums);

        assert_eq!(result, 4);
    }
}
