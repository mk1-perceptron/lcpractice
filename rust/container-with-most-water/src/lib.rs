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
