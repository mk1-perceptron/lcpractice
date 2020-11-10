/// Product of Array Except Self
///
/// Given an array nums of n integers where n > 1, return an array output such that output[i] is
/// equals to the product of all the elements of nums except nums[i].
///
/// Constraint: It's guaranteed that the product of the elements of any prefix or suffix of the
/// array (including the whole array) fits in a 32 bit integer.
///
/// Note: Please solve it without division and in O(n).
///
/// Follow up:
/// Could you solve it with constant space complexity? (The output array does not count as extra
/// space for the pupose of space complexity analysis.)

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    println!("{}", 0f64.log10());
    let total_product: i32 = nums.iter().product();
    let log_product = (total_product as f64).log2();

    nums.iter()
        .map(|&val| {
            if val == 0 {
                total_product
            } else {
                2f64.powf(log_product - (val as f64).log2()) as i32
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![1, 2, 3, 4];
        let result = product_except_self(input);

        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    #[test]
    fn all_zero() {
        let input = vec![0, 0];
        let result = product_except_self(input);

        assert_eq!(result, vec![0, 0]);
    }

    #[test]
    fn contains_zero() {
        let input = vec![1, 0];
        let result = product_except_self(input);

        assert_eq!(result, vec![0, 1]);
    }
}
