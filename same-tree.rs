// https://leetcode.com/problems/same-tree/

///////////////////////////////////////// V2 /////////////////////////////

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

struct TreeIterator {
    end: bool,
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>
}

impl TreeIterator {
    pub fn new(node_rc: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeIterator { 
            end: false,
            stack: vec![node_rc] 
        }
    }
}

// Implement Iterator for the secondary tracking struct
impl Iterator for TreeIterator {
    type Item = i32; // Returns references to the elements

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(leaf) = self.stack.pop() {
            if let Some(node_rc) = leaf {
                let node = node_rc.borrow();
                self.stack.push(node.left.clone());
                self.stack.push(node.right.clone());
                Some(node.val)
            } else {
                None
            }
        } else {
            self.end = true;
            None
        }
    }
}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut p_iterator = TreeIterator::new(p.clone());
        let mut q_iterator = TreeIterator::new(q.clone());
        
        loop {
            let p_node_rc = p_iterator.next();
            let q_node_rc = q_iterator.next();

            if p_node_rc != q_node_rc {
                return false;
            }

            if p_iterator.end && q_iterator.end {
                break;
            }
        }        

        true
    }
}



////////////////////////////////// BEST ANSWER LOL /////////////////

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
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }
}