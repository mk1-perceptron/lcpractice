//! Single Number
//!
//! Given a non-empty array of integers, every element appears twice except for one. Find that
//! single one.
//!
//! Note:
//! Your algorithm should have a linear runtime complexity. Could you implement it without using
//! extra memory?

pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, val| acc ^ val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 2, 1];
        let result = single_number(nums);

        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let nums = vec![4, 1, 2, 1, 2];
        let result = single_number(nums);

        assert_eq!(result, 4);
    }
}
