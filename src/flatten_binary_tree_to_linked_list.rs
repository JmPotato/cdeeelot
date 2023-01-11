/// 114.[flatten-binary-tree-to-linked-list](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list)
use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut root = root.as_ref().unwrap().borrow_mut();
        Solution::flatten(&mut root.left);
        Solution::flatten(&mut root.right);
        // Take out the left and insert between root and right.
        if root.left.is_none() {
            return;
        }
        let first = root.left.take().unwrap();
        let mut last = Rc::clone(&first);
        // Iterate to the last right node of the left node.
        while last.borrow().right.is_some() {
            let right = last.borrow().right.clone().unwrap();
            last = right;
        }
        let right = root.right.take();
        last.borrow_mut().right = right;
        root.right = Some(first);
    }
}
