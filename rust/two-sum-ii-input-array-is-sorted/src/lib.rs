//! Two Sum II - Input array is sorted
//!
//! Given an array of integers that is already sorted in ascending order, find two numbers such
//! that they add up to a specific target number.
//! The function twoSum should return indices of the two numbers such that they add up to the
//! target, where index1 must be less than index2.
//!
//! Note:
//! - Your returned answers (both index1 and index2) are not zero-base.
//! - You may assume that each inputb would have exactly one solution and you may not use the same
//!   element twice.

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let sum = numbers[left] + numbers[right];

        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            return vec![(left + 1) as i32, (right + 1) as i32];
        }
    }

    vec![0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(input, target);

        assert_eq!(result, vec![1, 2]);
    }
}
