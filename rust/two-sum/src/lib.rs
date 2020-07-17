//! Two Sum
//!
//! Given an array of integers, return indices of the two numbers such that they add up to a
//! specific target.
//! You may assume that each input would have exactly one solution, and you may not use the same
//! element twice.

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut remainders = HashMap::with_capacity(nums.len());

    for (index, val) in nums.iter().enumerate() {
        match remainders.get(val) {
            Some(&i) => return vec![i as i32, index as i32],
            None => {
                remainders.insert(target - val, index);
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(input, target);

        assert!(result.len() == 2);
        assert!(result[0] == 0);
        assert!(result[1] == 1);
    }

    #[test]
    fn example_two() {
        let input = vec![2, 1, 0, -1, -2];
        let target = 0;
        let result = two_sum(input, target);

        assert!(result.len() == 2);
        assert!(result[0] == 1);
        assert!(result[1] == 3);
    }
}
