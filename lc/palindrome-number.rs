//! rustc palindrome-number.rs --test
//! ./palindrome-number.exe

fn main() {}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }

        let mut aux = x;
        let mut characteristic = (x as f64).log10() as u32;

        while aux > 9 {
            let antilog = 10_i32.pow(characteristic);
            let first = aux / antilog;
            let last = aux % 10;

            if first != last { return false; }
            if characteristic < 2 { break; }

            aux = (aux - first * antilog) / 10;
            characteristic -= 2;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(x: i32, want: bool) {
        let got = Solution::is_palindrome(x);
        eprintln!("got is_palindrome({}) = {}, want {}", x, got, want);
        assert_eq!(got, want);
    }

    #[test]
    fn test_is_palindrome_example_1() { helper(121, true); }

    #[test]
    fn test_is_palindrome_example_2() { helper(-121, false); }

    #[test]
    fn test_is_palindrome_example_3() { helper(10, false); }

    #[test]
    fn test_is_palindrome_example_4() { helper(-101, false); }

    #[test]
    fn test_is_palindrome_high_even() { helper(1234554321, true); }

    #[test]
    fn test_is_palindrome_high_odd() { helper(123454321, true); }

}
