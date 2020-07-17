//! 3Sum
//!
//! Given an array of n integers, are there elements a, b, c, in nums such that a + b + c = 0?
//! Find all unique triplets in the array which gives the sum of zero.
//!
//! Note:
//! The solution set must not contain duplicate triplets

use std::{
    cmp,
    collections::{HashMap, HashSet},
};

pub fn three_sum_hash(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut found: HashSet<(i32, i32)> = HashSet::new();
    let mut dups: HashSet<i32> = HashSet::new();
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for i in 0..nums.len() {
        if dups.contains(&nums[i]) {
            continue;
        }
        dups.insert(nums[i]);

        for j in (i + 1)..nums.len() {
            let complement = -nums[i] - nums[j];

            if seen
                .get(&complement)
                .map_or_else(|| false, |&index| index == i)
            {
                let v1 = cmp::min(nums[i], cmp::min(complement, nums[j]));
                let v2 = cmp::max(nums[i], cmp::max(complement, nums[j]));
                if found.insert((v1, v2)) {
                    result.push(vec![nums[i], complement, nums[j]]);
                }
            }
            seen.insert(nums[j], i);
        }
    }

    result
}

pub fn three_sum_two_pointers(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    nums.sort();

    let mut result = Vec::new();

    for i in 0..nums.len() {
        if nums[i] > 0 {
            break;
        }

        if i == 0 || nums[i - 1] != nums[i] {
            two_sum(&nums, i, &mut result);
        }
    }

    result
}

fn two_sum(nums: &[i32], i: usize, result: &mut Vec<Vec<i32>>) {
    let mut left = i + 1;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[i] + nums[left] + nums[right];

        if sum < 0 || (left > i + 1 && nums[left] == nums[left - 1]) {
            left += 1;
        } else if sum > 0 || (right < nums.len() - 1 && nums[right] == nums[right + 1]) {
            right -= 1;
        } else {
            result.push(vec![nums[i], nums[left], nums[right]]);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum_hash(input);

        assert_eq!(result, vec![vec![-1, 0, 1], vec![-1, 2, -1]]);
    }

    #[test]
    fn example_one_two_pointer() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum_two_pointers(input);

        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
