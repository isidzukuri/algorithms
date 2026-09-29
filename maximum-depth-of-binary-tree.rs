https://leetcode.com/problems/maximum-depth-of-binary-tree/

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        let mut stack = vec![];
        let mut cur = root;
        let mut counter = 0;

        while cur.is_some() || !stack.is_empty()  {
            while let Some(node_rc) = cur {
                counter +=1;
                stack.push((Rc::clone(&node_rc), counter));
                cur = node_rc.borrow().left.clone();
            }

            if max < counter {
                max = counter;
            }

            if let Some((node_rc, cnt)) = stack.pop() {
                let node = node_rc.borrow();
                cur = node.right.clone();
                counter = cnt;
            }
        }
        max
    }
}