//! rustc valid-parentheses.rs --test
//! ./valid-parentheses.exe

fn main() {}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() < 2 || s.len() % 2 != 0 { return false; }

        let mut chars: Vec<char> = Vec::new();

        for x in s.chars() {
            match chars.last() {
                None => chars.push(x),
                Some(v) => match v {
                    '(' => if x == ')' { chars.pop(); } else { chars.push(x); },
                    '[' => if x == ']' { chars.pop(); } else { chars.push(x); },
                    '{' => if x == '}' { chars.pop(); } else { chars.push(x); },
                    _ => { return false; },
                },
            };
        }

        chars.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(s: &str, want: bool) {
        let s = s.to_string();
        let got = Solution::is_valid(s);
        assert_eq!(got, want);
    }

    #[test]
    fn test_is_valid_example_1() {
        helper("()", true);
    }

    #[test]
    fn test_is_valid_example_2() {
        helper("()[]{}", true);
    }

    #[test]
    fn test_is_valid_example_3() {
        helper("(]", false);
    }

    #[test]
    fn test_is_valid_example_4() {
        helper("([)]", false);
    }

    #[test]
    fn test_is_valid_example_5() {
        helper("{[]}", true);
    }

    #[test]
    fn test_is_valid_example_6() {
        helper("{{}[][[[]]]}", true);
    }

    #[test]
    fn test_is_valid_example_7() {
        helper("{{({})}}", true);
    }

}
