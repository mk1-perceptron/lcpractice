/// Valid Parentheses
///
/// Given a string containing just the characters '(', ')', '{', '}', '[', and ']', determine if
/// the input string is valid.
///
/// An input string is valid if:
/// 1. Open brackets must be close by the same type of brackets.
/// 2. Open brackets must be closed in the correct order.
///
/// Note that an empty string is also considered valid.

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::with_capacity(s.len() / 2);

    for c in s.chars() {
        if let Some(o) = get_opening(c) {
            if Some(o) != stack.pop() {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}

fn get_opening(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        '}' => Some('{'),
        ']' => Some('['),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "()".to_owned();
        let result = is_valid(input);

        assert_eq!(result, true);
    }

    #[test]
    fn example_two() {
        let input = "()[]{}".to_owned();
        let result = is_valid(input);

        assert_eq!(result, true);
    }

    #[test]
    fn example_three() {
        let input = "(]".to_owned();
        let result = is_valid(input);

        assert_eq!(result, false);
    }

    #[test]
    fn example_four() {
        let input = "([)]".to_owned();
        let result = is_valid(input);

        assert_eq!(result, false);
    }

    #[test]
    fn example_five() {
        let input = "{[]}".to_owned();
        let result = is_valid(input);

        assert_eq!(result, true);
    }

    #[test]
    fn single_open() {
        let input = "[".to_owned();
        let result = is_valid(input);

        assert_eq!(result, false);
    }
}
