//! rustc longest-common-prefix.rs --test
//! ./longest-common-prefix.exe

fn main() {}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let shortest: &str = strs
            .iter()
            .reduce(|a, b| if a.len() < b.len() { a } else { b })
            .unwrap();

        let idx = Solution::last_idx(&strs, shortest);

        shortest[..idx].to_string()
    }

    fn last_idx(strs: &Vec<String>, shortest: &str) -> usize {
        let mut idx = 0;
        for (i, a) in shortest.chars().enumerate() {
            for s in strs {
                if s.chars().nth(i).unwrap() != a { return idx; }
            }
            idx += 1;
        }
        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(strs: Vec<&str>, want: &str) {
        let strs: Vec<String> = strs.iter().map(|x| x.to_string()).collect();
        let got = Solution::longest_common_prefix(strs);
        assert_eq!(got, want);
    }

    #[test]
    fn test_longest_common_prefix_example_1() {
        helper(vec!["flower", "flow", "flight"], "fl");
    }

    #[test]
    fn test_longest_common_prefix_example_2() {
        helper(vec!["dog", "racecar", "car"], "");
    }

    #[test]
    fn test_longest_common_prefix_empty() {
        helper(vec![""], "");
    }

}
