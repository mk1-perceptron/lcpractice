fn main() {
    println!("Hello, world!");
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if let 0 | 1 = nums.len() {
        return nums.len() as i32;
    }

    let mut end = 1usize;
    let mut prev = nums[0];

    for i in 1..nums.len() {
        let cur = nums[i];
        if cur != prev {
            nums[end] = cur;
            end += 1;
        }
        prev = cur;
    }

    end as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut input = vec![1, 1, 2];
        let result = remove_duplicates(&mut input);
        println!("{:?}", input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_two() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = remove_duplicates(&mut input);
        println!("{:?}", input);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_empty() {
        let mut input = vec![];
        let result = remove_duplicates(&mut input);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_single() {
        let mut input = vec![0];
        let result = remove_duplicates(&mut input);

        assert_eq!(result, 1);
    }
}
