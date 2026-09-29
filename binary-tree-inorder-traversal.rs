// https://leetcode.com/problems/binary-tree-inorder-traversal/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = vec![];

        let mut current = root;
        
        while current.is_some() || !stack.is_empty() {
            while let Some(node_rc) = current {
                stack.push(     node_rc));
                current = node_rc.borrow().left.clone();
            }

            if let Some(node_rc) = stack.pop() {
                let node = node_rc.borrow();
                result.push(node.val);
                current = node.right.clone();
            }
        }
        result
    }
}