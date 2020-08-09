/// Reorder Data in Log Files
///
/// You have an array of logs. Each log is a space delimited string of words.
///
/// For each log, the first word in each log is an alphanumeric identifier. Then, either:
/// - Each word after the identifier will consist only of lowercase letters, or;
/// - Each word after the identifier will consist only of digits.
///
/// We will call these two vaarieties of logs letter-logs and digit-logs. It is guaranteed that
/// each log has at least one word after its identifier.
///
/// Reorder the logs so that all the letter-logs come before any digit-log. The letter-logs are
/// ordered lexicographically ignoring identifier, with the identifier used in case of ties. The
/// digit-logs should be put in their original order
///
/// Return the final order of the logs.
///
/// Constraints:
/// 1. 0 <= logs.length <= 100
/// 2. 3 <= logs[i].length <= 100
/// 3. logs[i] is guaranteed to have an identifier, and a word after the identifier
use std::cmp::Ordering;

pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
    let mut logs = logs;
    logs.sort_by(|a, b| {
        let (ident_a, log_a) = a.split_at(a.find(' ').unwrap_or(0));
        let (ident_b, log_b) = b.split_at(b.find(' ').unwrap_or(0));

        let is_digit_a = log_a.chars().skip(1).take(1).all(|c| c.is_digit(10));
        let is_digit_b = log_b.chars().skip(1).take(1).all(|c| c.is_digit(10));

        if !(is_digit_a || is_digit_b) {
            if log_a == log_b {
                ident_a.cmp(ident_b)
            } else {
                log_a.cmp(log_b)
            }
        } else if is_digit_a && is_digit_b {
            Ordering::Greater
        } else if is_digit_b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    logs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![
            "dig1 8 1 5 1".to_owned(),
            "let1 art can".to_owned(),
            "dig2 3 6".to_owned(),
            "let2 own kit dig".to_owned(),
            "let3 art zero".to_owned(),
        ];
        let result = reorder_log_files(input);

        assert_eq!(
            result,
            vec![
                "let1 art can",
                "let3 art zero",
                "let2 own kit dig",
                "dig1 8 1 5 1",
                "dig2 3 6"
            ]
        );
    }
}
