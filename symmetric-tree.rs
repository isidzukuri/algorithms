// https://leetcode.com/problems/symmetric-tree/

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node_rc) = root {
            let node = node_rc.borrow();
            is_symmetric(&node.left, &node.right)
        } else {
            true
        }
    }
}

fn is_symmetric(left_opt: &Option<Rc<RefCell<TreeNode>>>, right_opt: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left_opt, right_opt) {
        (Some(left_rc), Some(right_rc)) => {
            let left = left_rc.borrow();
            let right = right_rc.borrow();

            left.val == right.val &&
            is_symmetric(&left.left, &right.right) &&
            is_symmetric(&left.right, &right.left) 
        }
        (None, None) => { true },
        _ => { false }
    }
}


//////////////////////////////////// AI 

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root_node = match root {
            Some(node) => node,
            None => return true,
        };

        let mut queue = VecDeque::new();
        {
            let node = root_node.borrow();
            queue.push_back(node.left.clone());
            queue.push_back(node.right.clone());
        }

        while let (Some(left), Some(right)) = (queue.pop_front(), queue.pop_front()) {
            match (left, right) {
                (None, None) => continue,
                (Some(l_rc), Some(r_rc)) => {
                    let l = l_rc.borrow();
                    let r = r_rc.borrow();

                    if l.val != r.val {
                        return false;
                    }

                    queue.push_back(l.left.clone());
                    queue.push_back(r.right.clone());
                    queue.push_back(l.right.clone());
                    queue.push_back(r.left.clone());
                }
                _ => return false,
            }
        }

        true
    }
}