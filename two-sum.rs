//! rustc two-sum.rs --test
//! ./two-sum.exe

use std::collections::HashMap;

fn main() {}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idxs = HashMap::new();
        for (j, num) in nums.iter().enumerate() {
            match idxs.get(&(target - num)) {
                Some(i) => return vec![*i, j as i32],
                None => idxs.insert(num, j as i32),
            };
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(nums: Vec<i32>, target: i32, want: Vec<i32>) {
        let got = Solution::two_sum(nums, target);
        assert_eq!(got, want);
    }

    #[test]
    fn test_two_sum_example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let want = vec![0, 1];
        helper(nums, target, want);
    }

    #[test]
    fn test_two_sum_example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let want = vec![1, 2];
        helper(nums, target, want);
    }

    #[test]
    fn test_two_sum_example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let want = vec![0, 1];
        helper(nums, target, want);
    }

    #[test]
    fn test_two_sum_empty() {
        let nums = vec![];
        let target = 5;
        let want = vec![];
        helper(nums, target, want);
    }

    #[test]
    fn test_two_sum_no_result() {
        let nums = vec![1, 2, 3, 4];
        let target = 15;
        let want = vec![];
        helper(nums, target, want);
    }

}
