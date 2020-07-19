/// Integer to Roman
///
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D, and M.
///
/// Symbol      Value
/// I           1
/// V           5
/// X           10
/// L           50
/// C           100
/// D           500
/// M           1000
///
/// For example, two is written as II in Roman numeral, just two one's added together. Twelve is
/// written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is
/// XX + V + II.
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral
/// for four is not IIII. Instead, the number four is written as IV. Because the one is before the
/// five we subtract it make four. The same principle applies to the number nine, which is written
/// as IX. There are six instances where subtraction is used:
/// - I can be placed before V (5) and X (10) to make 4 and 9.
/// - X can be placed before L (50) and C (100) to make 40 and 90.
/// - C can be palced before D (500) and M (1000) to make 400 and 900.
/// Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range
/// from 1 to 3999.

pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut rev_num = Vec::new();

    handle_digit(&mut rev_num, 'I', 'V', 'X', num % 10);
    num /= 10;
    handle_digit(&mut rev_num, 'X', 'L', 'C', num % 10);
    num /= 10;
    handle_digit(&mut rev_num, 'C', 'D', 'M', num % 10);
    num /= 10;
    handle_digit(&mut rev_num, 'M', 'V', '!', num % 10);

    rev_num.iter().rev().collect()
}

fn handle_digit(rev_num: &mut Vec<char>, one: char, five: char, ten: char, digit: i32) {
    match digit {
        0..=3 => {
            for _ in 0..digit {
                rev_num.push(one);
            }
        }
        4 => {
            rev_num.push(five);
            rev_num.push(one);
        }
        5..=8 => {
            for _ in 0..(digit - 5) {
                rev_num.push(one);
            }
            rev_num.push(five);
        }
        9 => {
            rev_num.push(ten);
            rev_num.push(one);
        }
        n => panic!("Expected a single digit, got {}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = 3;
        let result = int_to_roman(input);

        assert_eq!(result, "III");
    }

    #[test]
    fn example_two() {
        let input = 4;
        let result = int_to_roman(input);

        assert_eq!(result, "IV");
    }

    #[test]
    fn example_three() {
        let input = 9;
        let result = int_to_roman(input);

        assert_eq!(result, "IX");
    }

    #[test]
    fn example_four() {
        let input = 58;
        let result = int_to_roman(input);

        assert_eq!(result, "LVIII");
    }

    #[test]
    fn example_five() {
        let input = 1994;
        let result = int_to_roman(input);

        assert_eq!(result, "MCMXCIV");
    }
}
