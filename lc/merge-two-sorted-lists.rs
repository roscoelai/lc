// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn from_vec(v: Vec<i32>) -> Option<Box<Self>> {
        let mut n = None;
        for x in v.iter().rev() {
            n = Some(Box::new(Self { val: *x, next: n } ));
        }
        n
    }
}

trait ToVec {
    fn to_vec(&self) -> Vec<i32>;
}

impl ToVec for Option<Box<ListNode>> {
    fn to_vec(&self) -> Vec<i32> {
      let mut res = Vec::new();
      let mut n = self;
      while let Some(v) = n {
          res.push(v.val);
          n = &v.next;
      }
      res
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>, 
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>>
    {
        match (l1, l2) {
            (Some(a), Some(b)) => Some(Box::new(
                if a.val <= b.val {
                    ListNode {
                        val: a.val,
                        next: Self::merge_two_lists(a.next, Some(b))
                    }
                } else {
                    ListNode {
                        val: b.val,
                        next: Self::merge_two_lists(Some(a), b.next)
                    }
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

    fn helper(v1: Vec<i32>, v2: Vec<i32>, want: Vec<i32>) {
        let l1 = ListNode::from_vec(v1.clone());
        let l2 = ListNode::from_vec(v2.clone());
        let got = Solution::merge_two_lists(l1, l2).to_vec();
        eprintln!("merge_two_lists({:?}, {:?}) = {:?}", v1, v2, got);
        assert_eq!(got, want);
    }

    #[test]
    fn test_merge_two_lists_1() {
        helper(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_two_lists_2() {
        helper(vec![], vec![], vec![]);
    }

    #[test]
    fn test_merge_two_lists_3() {
        helper(vec![], vec![0], vec![0]);
    }

}
