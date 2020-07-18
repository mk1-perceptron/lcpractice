//! Roman to Integer
//!
//! Roman numerals are represented by seven different symbols: I, V, X, L, C, D, and M.
//!
//! Symbol      Value
//! I           1
//! V           5
//! X           10
//! L           50
//! C           100
//! D           500
//! M           1000
//!
//! For example, two is written as II in Roman numberal, just two one's added together. Twelve is
//! written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is
//! XX + V + II.
//! Roman Numerals are usually written largest to smallest from left to right. However, the numeral
//! for four is not IIII. Instead, the number four is written as IV. Because the one is before the
//! five we subtract it make four. The same principle applies to the number nine, which is written
//! as IX. There are six instances where subtraction is used:
//! - I can be place before V (5) and X (10) to make 4 and 9.
//! - X can be place before L (50) and C (100) to make 40 and 90.
//! - C can be placed before D (500) and M (1000) to make 400 and 900.
//! Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range
//! from 1 to 3999.

pub fn roman_to_int(s: String) -> i32 {
    s.chars()
        .fold(('~', 0), |(p, mut t), c| {
            match (p, c) {
                (_, 'I') => t += 1,
                ('I', 'V') => t += 4 - 1,
                (_, 'V') => t += 5,
                ('I', 'X') => t += 9 - 1,
                (_, 'X') => t += 10,
                ('X', 'L') => t += 40 - 10,
                (_, 'L') => t += 50,
                ('X', 'C') => t += 90 - 10,
                (_, 'C') => t += 100,
                ('C', 'D') => t += 400 - 100,
                (_, 'D') => t += 500,
                ('C', 'M') => t += 900 - 100,
                (_, 'M') => t += 1000,
                (_, _) => {}
            }
            (c, t)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "III".to_owned();
        let result = roman_to_int(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let input = "IV".to_owned();
        let result = roman_to_int(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn example_three() {
        let input = "IX".to_owned();
        let result = roman_to_int(input);

        assert_eq!(result, 9);
    }

    #[test]
    fn example_four() {
        let input = "LVIII".to_owned();
        let result = roman_to_int(input);

        assert_eq!(result, 58);
    }

    #[test]
    fn example_five() {
        let input = "MCMXCIV".to_owned();
        let result = roman_to_int(input);

        assert_eq!(result, 1994);
    }
}
