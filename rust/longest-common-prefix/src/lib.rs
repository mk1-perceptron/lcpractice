//! Longest Common Prefix
//!
//! Write a function to find the longest common prefix string amongst an array of strings.
//! If there is no common prefix, return an empty string "".

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_owned();
    }

    let mut common = String::new();
    let mut iters = strs.iter().map(|s| s.chars()).collect::<Vec<_>>();

    for _ in 0..strs[0].len() {
        let chars = iters.iter_mut().map(|i| i.next()).collect::<Vec<_>>();
        let c1 = chars[0].unwrap();

        if chars.iter().all(|o| match o {
            Some(c) => *c == c1,
            None => false,
        }) {
            common.push(c1);
        } else {
            break;
        }
    }

    common
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()];
        let result = longest_common_prefix(input);

        assert_eq!(result, "fl");
    }

    #[test]
    fn example_two() {
        let input = vec!["dog".to_owned(), "racecar".to_owned(), "car".to_owned()];
        let result = longest_common_prefix(input);

        assert_eq!(result, "");
    }

    #[test]
    fn single_string() {
        let input = vec!["a".to_owned()];
        let result = longest_common_prefix(input);

        assert_eq!(result, "a");
    }
}
