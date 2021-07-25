//! rustc palindrome-number.rs --test
//! ./palindrome-number.exe

fn main() {}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }

        let mut aux = x;
        let mut carrys = (aux as f64).log10() as u32;

        while aux > 9 {
            let tens = 10_i32.pow(carrys);
            let first = aux / tens;
            let last = aux % 10;

            if first != last { return false; }

            aux = (aux - first * tens) / 10;
            println!("carrys = {}", carrys);
            carrys = match carrys.checked_sub(2) {
                Some(v) => v,
                None => break,
            };
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(x: i32, want: bool) {
        let got = Solution::is_palindrome(x);
        assert_eq!(got, want);
    }

    #[test]
    fn test_is_palindrome_example_1() {
        helper(121, true);
    }

    #[test]
    fn test_is_palindrome_example_2() {
        helper(-121, false);
    }

    #[test]
    fn test_is_palindrome_example_3() {
        helper(10, false);
    }

    #[test]
    fn test_is_palindrome_example_4() {
        helper(-101, false);
    }

    #[test]
    fn test_is_palindrome_high_even() {
        helper(2109889012, true);
    }

    #[test]
    fn test_is_palindrome_high_odd() {
        helper(210989012, true);
    }

    #[test]
    fn test_is_palindrome_zero() {
        helper(0, true);
    }

}
