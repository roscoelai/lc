struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut nums: Vec<i32> = s
            .chars()
            .map(|x| match x {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Invalid Roman numeral: '{}'", s),
            })
            .collect();

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

    fn helper(s: &str, want: i32) {
        let got = Solution::roman_to_int(s.to_string());
        assert_eq!(got, want);
    }

    #[test]
    fn test_roman_to_int_1() { helper("III", 3); }

    #[test]
    fn test_roman_to_int_2() { helper("IV", 4); }

    #[test]
    fn test_roman_to_int_3() { helper("IX", 9); }

    #[test]
    fn test_roman_to_int_4() { helper("LVIII", 58); }

    #[test]
    fn test_roman_to_int_5() { helper("MCMXCIV", 1994); }

    #[test]
    fn test_roman_to_int_min() { helper("I", 1); }

    #[test]
    fn test_roman_to_int_max() { helper("MMMCMXCIX", 3999); }

    #[test]
    #[should_panic(expected = "Invalid Roman numeral: 'MMMCMZXCIX'")]
    fn test_roman_to_int_invalid() {
        Solution::roman_to_int("MMMCMZXCIX".to_string());
    }

}
