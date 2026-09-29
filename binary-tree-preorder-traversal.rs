// https://leetcode.com/problems/binary-tree-preorder-traversal/


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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        if let Some(node) = root {
            stack.push(node);
        }

        while let Some(node_rc) = stack.pop() {
            let node = node_rc.borrow();
            result.push(node.val);

            if let Some(right) = &node.right {
                stack.push(Rc::clone(right));
            }
            if let Some(left) = &node.left {
                stack.push(Rc::clone(left));
            }
        }

        result
    }
}



impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];
        let mut cur = root;
        
        while cur.is_some() || !stack.is_empty() {
            while let Some(node_rc) = cur {
                let node = node_rc.borrow();
                result.push(node.val);

                if let Some(right) = &node.right {
                    stack.push(Rc::clone(right));
                }

                cur = node.left.clone();
            }
            cur = stack.pop();
        }
        result
    }
}



use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![root];

        while let Some(cur) = stack.pop() {
            if let Some(node_rc) = cur {
                let node = node_rc.borrow(); 
                result.push(node.val);
                stack.push(node.right.clone());
                stack.push(node.left.clone());
            }
        }
        result
    }
}


impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        
        if let Some(root_node) = root {
            let mut stack = vec![root_node];
            while let Some(node_rc) = stack.pop() {
                let node = node_rc.borrow(); 
                result.push(node.val);
                if let Some(right) = &node.right {
                    stack.push(Rc::clone(&right));
                }
                if let Some(left) = &node.left {
                    stack.push(Rc::clone(&left));
                }
            }
        }
        result
    }
}