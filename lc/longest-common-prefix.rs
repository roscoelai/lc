struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let n = strs.iter().map(|x| x.len()).min().unwrap();
        let mut res = String::with_capacity(n);

        for i in 0..n {
            let c = &strs[0][i..=i];

            for j in 1..strs.len() {
                if &strs[j][i..=i] != c {
                    return res;
                }
            }

            res.push_str(c);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(strs: Vec<&str>, want: &str) {
        let strs: Vec<String> = strs.iter().map(|x| x.to_string()).collect();
        let got = Solution::longest_common_prefix(strs.clone());
        eprintln!("longest_common_prefix({:?}) = {:?}", strs, got);
        assert_eq!(got, want);
    }

    #[test]
    fn test_longest_common_prefix_1() {
        helper(vec!["flower", "flow", "flight"], "fl");
    }

    #[test]
    fn test_longest_common_prefix_2() {
        helper(vec!["dog", "racecar", "car"], "");
    }

    #[test]
    fn test_longest_common_prefix_empty() {
        helper(vec![""], "");
    }

}
