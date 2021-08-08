struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut aux = match x.checked_abs() {
            Some(v) => v,
            None => return 0,
        };
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
    fn test_reverse_1() { helper(123, 321); }

    #[test]
    fn test_reverse_2() { helper(-123, -321); }

    #[test]
    fn test_reverse_3() { helper(120, 21); }

    #[test]
    fn test_reverse_4() { helper(0, 0); }

    #[test]
    fn test_reverse_max() { helper(2147483647, 0); }

    #[test]
    fn test_reverse_min() { helper(-2147483648, 0); }

}
