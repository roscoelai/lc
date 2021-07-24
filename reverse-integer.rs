//! rustc reverse-integer.rs --test
//! ./reverse-integer.exe

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut aux = x.abs();
        let mut res: i32 = 0;

        while aux > 0 {
            res = match res.checked_mul(10) {
                Some(v) => v,
                None => return 0,
            };
            res = match res.checked_add(aux % 10) {
                Some(v) => v,
                None => return 0,
            };
            aux /= 10;
        }

        if x < 0 { return -res; }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(x: i32, want: i32) {
        let got = Solution::reverse(x);
        assert_eq!(got, want);
    }

    #[test]
    fn test_reverse_example_1() {
        helper(123, 321);
    }

    #[test]
    fn test_reverse_example_2() {
        helper(-123, -321);
    }

    #[test]
    fn test_reverse_example_3() {
        helper(120, 21);
    }

    #[test]
    fn test_reverse_example_4() {
        helper(0, 0);
    }

    #[test]
    fn test_reverse_over() {
        helper(1123456789, 0);
    }

    #[test]
    fn test_reverse_under() {
        helper(-1123456789, 0);
    }

}
