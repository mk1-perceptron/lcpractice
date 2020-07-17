//! Longest Substring Without Repeating Characters
//!
//! Given a string, find the length of the longest substring without repeating characters.

use std::cmp;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut i = 0;
    let mut max = 0i32;
    let mut seen = HashMap::with_capacity(s.len());

    for (j, b) in s.bytes().enumerate() {
        let c = b as char;

        if let Some(&l) = seen.get(&c) {
            i = cmp::max(l, i);
        }

        max = cmp::max(max, (j - i + 1) as i32);
        seen.insert(c, j + 1);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "abcabcbb";
        let result = length_of_longest_substring(input.to_owned());

        assert_eq!(result, 3);
    }

    #[test]
    fn test_two() {
        let input = "bbbbb";
        let result = length_of_longest_substring(input.to_owned());

        assert_eq!(result, 1);
    }

    #[test]
    fn test_three() {
        let input = "pwwkew";
        let result = length_of_longest_substring(input.to_owned());

        assert_eq!(result, 3);
    }
}
