//! rustc add-two-numbers.rs --test
//! ./add-two-numbers.exe

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

  fn from_vec(v: Vec<i32>) -> Option<Box<Self>> {
      let mut n = None;
      for x in v.iter().rev() {
          n = Some(Box::new(ListNode { val: *x, next: n } ));
      }
      n
  }

  fn to_vec(ln: Option<Box<Self>>) -> Vec<i32> {
      let mut res = Vec::new();
      let mut n = ln;
      while let Some(v) = n {
          res.push(v.val);
          n = v.next;
      }
      res
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>, 
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        Self::merge_two(l1, l2, 0)
    }

    fn merge_two(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32
    ) -> Option<Box<ListNode>> {
        Some(Box::new(match (l1, l2) {
            (Some(a), Some(b)) => {
                let sum = a.val + b.val + carry;
                ListNode {
                    val: sum % 10,
                    next: Self::merge_two(a.next, b.next, sum / 10)
                }
            },
            (Some(v), None) | (None, Some(v)) => {
                let sum = v.val + carry;
                ListNode {
                    val: sum % 10,
                    next: Self::merge_two(v.next, None, sum / 10)
                }
            },
            _ if carry > 0 => ListNode::new(carry),
            _ => return None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(v1: Vec<i32>, v2: Vec<i32>, want: Vec<i32>) {
        let l1 = ListNode::from_vec(v1.clone());
        let l2 = ListNode::from_vec(v2.clone());
        let got = Solution::add_two_numbers(l1, l2);
        let got = ListNode::to_vec(got);
        eprintln!("got add_two_numbers({:?}, {:?}) = {:?}, want {:?}", 
                  v1, v2, got, want);
        assert_eq!(got, want);
    }

    #[test]
    fn test_add_two_numbers_example_1() {
        helper(vec![2, 4, 3], 
               vec![5, 6, 4], 
               vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_example_2() {
        helper(vec![0], 
               vec![0], 
               vec![0]);
    }

    #[test]
    fn test_add_two_numbers_example_3() {
        helper(vec![9, 9, 9, 9, 9, 9, 9], 
               vec![9, 9, 9, 9], 
               vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

}
