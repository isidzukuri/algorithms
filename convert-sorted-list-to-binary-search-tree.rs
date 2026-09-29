// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = &head;
        let mut values = vec![];

        while let Some(list_node) = head {
            values.push(list_node.val);
            head = &list_node.next;
        }

        build_tree(&values, 0, values.len()-1 )      
    }
}

pub fn build_tree(values: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>>{
    if values.is_empty() { return None }
    
    let mid = left + (right-left)/2;
    let mut root = TreeNode::new(values[mid]);

    if mid > left {
        root.left = build_tree(values, left, mid-1 );
    }
    if right > mid {
        root.right = build_tree(values, mid+1, right );
    }

    Some(Rc::new(RefCell::new(root)))
}




////////////////////////////////////////////////////// AI 

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Collect values into a Vector
        let mut values = Vec::new();
        while let Some(node) = head {
            values.push(node.val);
            head = node.next;
        }

        Self::build_tree(&values)
    }

    fn build_tree(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let mid = vals.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(vals[mid])));

        // Sub-slices naturally handle bounds without index underflow risks!
        root.borrow_mut().left = Self::build_tree(&vals[..mid]);
        root.borrow_mut().right = Self::build_tree(&vals[mid + 1..]);

        Some(root)
    }
}



use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 1. Calculate the length of the linked list
        let mut len = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        // 2. Build BST using in-order traversal
        let mut head_ptr = head;
        Self::build_tree_inorder(&mut head_ptr, 0, len)
    }

    fn build_tree_inorder(
        head: &mut Option<Box<ListNode>>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start >= end {
            return None;
        }

        let mid = start + (end - start) / 2;

        // Construct left subtree first
        let left = Self::build_tree_inorder(head, start, mid);

        // Process current root node from the front of the list
        let node_val = head.as_ref().unwrap().val;
        let root = Rc::new(RefCell::new(TreeNode::new(node_val)));
        root.borrow_mut().left = left;

        // Advance head pointer in the list
        *head = head.as_mut().unwrap().next.take();

        // Construct right subtree
        root.borrow_mut().right = Self::build_tree_inorder(head, mid + 1, end);

        Some(root)
    }
}