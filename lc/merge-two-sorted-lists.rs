//! rustc merge-two-sorted-lists.rs --test
//! ./merge-two-sorted-lists.exe

fn main() {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(a), Some(b)) => Some(Box::new(
                if a.val <= b.val {
                    ListNode { val: a.val, next: Self::merge_two_lists(a.next, Some(b)) }
                } else {
                    ListNode { val: b.val, next: Self::merge_two_lists(Some(a), b.next) }
                }
            )),
            (None, Some(v)) | (Some(v), None) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linked_list_head(l: Vec<i32>) -> Option<Box<ListNode>> {
        let mut last = ListNode::new({
            match l.last() {
                Some(v) => *v,
                None => return None,
            }
        });

        for x in l.iter().rev().skip(1) {
            let mut first = ListNode::new(*x);
            first.next = Some(Box::new(last));
            last = first;
        }

        Some(Box::new(last))
    }

    fn collect_to_vec(n: Option<Box<ListNode>>) -> Vec<i32> {
        let mut aux = n;
        let mut res: Vec<i32> = Vec::new();
        loop {
            match aux {
                None => break,
                Some(v) => {
                    res.push(v.val);
                    aux = v.next;
                },
            };
        }
        res
    }

    fn helper(l1: Vec<i32>, l2: Vec<i32>, want: Vec<i32>) {
        let l1 = linked_list_head(l1);
        let l2 = linked_list_head(l2);
        let got = Solution::merge_two_lists(l1, l2);
        let got = collect_to_vec(got);
        assert_eq!(got, want);
    }

    #[test]
    fn test_merge_two_lists_example_1() {
        helper(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_two_lists_example_2() {
        helper(vec![], vec![], vec![]);
    }

    #[test]
    fn test_merge_two_lists_example_3() {
        helper(vec![], vec![0], vec![0]);
    }

}
