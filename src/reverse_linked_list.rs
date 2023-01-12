// 206.[reverse-linked-list](https://leetcode.cn/problems/reverse-linked-list)

use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut last = Solution::reverse_list(head.as_ref().unwrap().next.clone()).unwrap();
        // Iterate to the current last node of the reversed list.
        // This is necessary since the last node is hold by a cloned `Option<Box<ListNode>>`,
        // which doesn't allow us to make an in-place change to `head`. So we need to reach
        // the last node of the reversed list and make the change there.
        let mut next = &mut last;
        while next.next.is_some() {
            next = next.next.as_mut().unwrap();
        }
        let mut head = head.unwrap();
        head.next = None;
        // Reverse the next node's next pointer to the current head.
        next.next = Some(head.clone());
        // Always return the last node to make sure the result will be started from the head
        // of the reversed list.
        Some(last)
    }
}
