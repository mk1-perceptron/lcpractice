//! Container With Most Water
//!
//! Given n non-negative integers a1, a2, ..., an, where each
//! represents a point at coordinate (i, ai). n vertical lines are
//! drawn such that the two endpoints of line i is at (i, ai) and (i, 0).
//! Find two lines, which together with x-axis forms a container, such
//! that the container contains the most water.
//!
//! Note:
//! You may not slant the container and n is at least 2.

use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        max = cmp::max(
            max,
            cmp::min(height[left], height[right]) * (right as i32 - left as i32),
        );
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = max_area(input);

        assert_eq!(result, 49);
    }
}
