fn main() {
    println!("Hello, world!");
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut x = x;
    let mut rev = 0;

    while x > rev {
        rev = rev * 10 + x % 10;
        x /= 10;
    }

    x == rev || x == rev / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let num = 121;
        let result = is_palindrome(num);

        assert_eq!(result, true);
    }

    #[test]
    fn test_two() {
        let num = -121;
        let result = is_palindrome(num);

        assert_eq!(result, false);
    }
}
