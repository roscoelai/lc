struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 { return false; }

        let mut stack = Vec::new();

        for x in s.chars() {
            stack.push(match x {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                ')' | ']' | '}' if Some(x) != stack.pop() => return false,
                ')' | ']' | '}' => continue,
                _ => return false,
            });
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(s: &str, want: bool) {
        let got = Solution::is_valid(s.to_string());
        eprintln!("is_valid({:?}) = {:?}", s, got);
        assert_eq!(got, want);
    }

    #[test]
    fn test_is_valid_1() { helper("()", true); }

    #[test]
    fn test_is_valid_2() { helper("()[]{}", true); }

    #[test]
    fn test_is_valid_3() { helper("(]", false); }

    #[test]
    fn test_is_valid_4() { helper("([)]", false); }

    #[test]
    fn test_is_valid_5() { helper("{[]}", true); }

    #[test]
    fn test_is_valid_6() { helper("{{}[][[[]]]}", true); }

    #[test]
    fn test_is_valid_7() { helper("{{({})}}", true); }

}
