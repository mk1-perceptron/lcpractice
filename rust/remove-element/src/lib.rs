//! Remove Element
//!
//! Given an array nums and a value val, remove all instances of that value in-place and return the
//! new length.
//! Do not allocate extra space for another array, you must do this by modifying the input array
//! in-place with O(1) extra memory.
//! The order of elements can be changed. It doesn't matter what you leave beyond the new length.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut end_index = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[end_index] = nums[i];
            end_index += 1;
        }
    }

    end_index as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let result = remove_element(&mut nums, val);

        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let result = remove_element(&mut nums, val);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_empty() {
        let mut nums = vec![];
        let val = 1;
        let result = remove_element(&mut nums, val);

        assert_eq!(result, 0);
    }
}
