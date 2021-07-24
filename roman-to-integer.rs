//! rustc roman-to-integer.rs --test
//! ./roman-to-integer.exe

use std::collections::HashMap;

fn main() {}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000)
        ].iter().cloned().collect();

        // if s.len() == 1 { return map[&s.chars().next().unwrap()]; }

        let mut nums: Vec<i32> = s.chars().map(|c| map[&c]).collect();

        for i in 1..s.len() {
            if nums[i - 1] < nums[i] {
                nums[i - 1] = -nums[i - 1];
            }
        }

        nums.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(s: String, want: i32) {
        let got = Solution::roman_to_int(s);
        assert_eq!(got, want);
    }

    #[test]
    fn test_roman_to_int_example_1() {
        helper("III".to_string(), 3);
    }

    #[test]
    fn test_roman_to_int_example_2() {
        helper("IV".to_string(), 4);
    }

    #[test]
    fn test_roman_to_int_example_3() {
        helper("IX".to_string(), 9);
    }

    #[test]
    fn test_roman_to_int_example_4() {
        helper("LVIII".to_string(), 58);
    }

    #[test]
    fn test_roman_to_int_example_5() {
        helper("MCMXCIV".to_string(), 1994);
    }

    #[test]
    fn test_roman_to_int_min() {
        helper("I".to_string(), 1);
    }

    #[test]
    fn test_roman_to_int_max() {
        helper("MMMCMXCIX".to_string(), 3999);
    }

}
