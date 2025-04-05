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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy.next;
        let mut addition = 0;

        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() || addition > 0 {
            let val1 = l1.as_ref().map_or(0, |n| n.val);
            let val2 = l2.as_ref().map_or(0, |n| n.val);
            let sum = val1 + val2 + addition;

            addition = sum / 10;
            *curr = Some(Box::new(ListNode::new(sum % 10)));
            curr = &mut curr.as_mut().unwrap().next;

            l1 = l1.and_then(|node|node.next);
            l2 = l2.and_then(|node|node.next);

        }
        dummy.next
    }
}