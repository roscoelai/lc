use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars = HashSet::new();
        let mut lengths = Vec::with_capacity(s.len());
        let mut length = 0;

        for c in s.chars() {
            if chars.insert(c) {
                length += 1;
            } else {
                lengths.push(length);
                chars.retain(|&x| x == c);
                length = 1;
            }
        }

        *lengths.iter().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(s: &str, want: i32) {
        let got = Solution::length_of_longest_substring(s.to_string());
        eprintln!("length_of_longest_substring({:?}) = {:?}", s, got);
        assert_eq!(got, want);
    }

    #[test]
    fn test_length_of_longest_substring_1() { helper("abcabcbb", 3); }

    #[test]
    fn test_length_of_longest_substring_2() { helper("bbbbb", 1); }

    #[test]
    fn test_length_of_longest_substring_3() { helper("pwwkew", 3); }

    #[test]
    fn test_length_of_longest_substring_4() { helper("", 0); }

}
