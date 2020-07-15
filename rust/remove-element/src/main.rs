fn main() {
    println!("Hello, world!");
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
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
    fn test_one() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let result = remove_element(&mut nums, val);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_two() {
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
