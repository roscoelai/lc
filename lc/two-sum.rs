use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idxs = HashMap::new();

        for (j, num) in nums.iter().enumerate() {
            let diff = target - num;
            match idxs.get(&diff) {
                None => idxs.insert(num, j as i32),
                Some(i) => return vec![*i, j as i32],
            };
        }

        panic!("Impossible. Guaranteed to have no valid answers.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(nums: Vec<i32>, target: i32, want: Vec<i32>) {
        let mut got = Solution::two_sum(nums.clone(), target);
        got.sort();
        eprintln!("two_sum({:?}, {:?}) = {:?}", nums, target, got);
        assert_eq!(got, want);
    }

    #[test]
    fn test_two_sum_1() { helper(vec![2, 7, 11, 15], 9, vec![0, 1]); }

    #[test]
    fn test_two_sum_2() { helper(vec![3, 2, 4], 6, vec![1, 2]); }

    #[test]
    fn test_two_sum_3() { helper(vec![3, 3], 6, vec![0, 1]); }

}
