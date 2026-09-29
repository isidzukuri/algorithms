// https://leetcode.com/problems/construct-string-from-binary-tree

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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() { return "".to_string() }

        let node_rc = root.unwrap();
        let node = node_rc.borrow();

        let left = Self::tree2str(node.left.clone());
        let right = Self::tree2str(node.right.clone());

        match (left.is_empty(), right.is_empty()) {
            (true, true) => node.val.to_string(),
            (true, false) => format!("{}()({})", node.val, right),
            (false, true) => format!("{}({})", node.val, left),
            (false, false) => format!("{}({})({})", node.val, left, right),
        }
    }
}





////////////////////////////////////////////////////// AI


use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        Self::traverse(&root, &mut result);
        result
    }

    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, acc: &mut String) {
        if let Some(node_rc) = node {
            let node = node_rc.borrow();
            
            // Append node value using std::fmt::Write or push_str
            acc.push_str(&node.val.to_string());

            // Left child is required if left exists OR right exists (to preserve mapping)
            if node.left.is_some() || node.right.is_some() {
                acc.push('(');
                Self::traverse(&node.left, acc);
                acc.push(')');
            }

            // Right child is only appended if it exists
            if node.right.is_some() {
                acc.push('(');
                Self::traverse(&node.right, acc);
                acc.push(')');
            }
        }
    }
} 