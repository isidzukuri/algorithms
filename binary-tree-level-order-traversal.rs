// https://leetcode.com/problems/binary-tree-level-order-traversal/


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
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut que = VecDeque::new();

        if let Some(node_rc) = root {
            que.push_back(node_rc);
        }

        while !que.is_empty() {
            let level_size = que.len();
            let mut level_result = Vec::with_capacity(level_size);

            for _ in (0..level_size) {
                if let Some(node_rc) = que.pop_front() {
                    let node = node_rc.borrow();
                    level_result.push(node.val);
                    if let Some(left) = &node.left {
                        que.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node.right {
                        que.push_back(Rc::clone(right));
                    }
                }    
            }
            result.push(level_result);
        }
        result
    }
}




use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut que = VecDeque::new();

        if let Some(node_rc) = root {
            que.push_back(node_rc);
        }
        let mut pops = 1;

        while !que.is_empty() {
            let mut level_result = vec![];
            let mut next_pops = 0;

            for _ in (0..pops) {
                if let Some(node_rc) = que.pop_front() {
                    let node = node_rc.borrow();
                    level_result.push(node.val);
                    if let Some(left) = &node.left {
                        que.push_back(Rc::clone(left));
                        next_pops +=1;
                    }
                    if let Some(right) = &node.right {
                        que.push_back(Rc::clone(right));
                        next_pops +=1;
                    }
                }    
            }
            result.push(level_result);
            pops = next_pops;
        }
        result
    }
}