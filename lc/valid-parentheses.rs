//! rustc valid-parentheses.rs --test
//! ./valid-parentheses.exe

fn main() {}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 { return false; }

        let mut stack = Vec::new();

        for x in s.chars() {
            match x {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if Some(x) != stack.pop() => return false,
                ')' | ']' | '}' => (),
                _ => return false,
            };
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(s: &str, want: bool) {
        let got = Solution::is_valid(s.to_string());
        eprintln!("got is_valid('{}') = {}, want {}", s, got, want);
        assert_eq!(got, want);
    }

    #[test]
    fn test_is_valid_example_1() { helper("()", true); }

    #[test]
    fn test_is_valid_example_2() { helper("()[]{}", true); }

    #[test]
    fn test_is_valid_example_3() { helper("(]", false); }

    #[test]
    fn test_is_valid_example_4() { helper("([)]", false); }

    #[test]
    fn test_is_valid_example_5() { helper("{[]}", true); }

    #[test]
    fn test_is_valid_example_6() { helper("{{}[][[[]]]}", true); }

    #[test]
    fn test_is_valid_example_7() { helper("{{({})}}", true); }

}
