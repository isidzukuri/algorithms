// https://leetcode.com/problems/remove-linked-list-elements

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


////////////////////////// V1 ////////////////////////////////

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;

        if let Some(ref mut node) = head {
            if node.val == val {
                head = Self::remove_elements(node.next.take(), val);
            } else {
                node.next = Self::remove_elements(node.next.take(), val);
            }
        }

        head
    }
}



////////////////////////// V2 ////////////////////////////////


impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = &mut head;
        loop {
            match cur {
                Some(node) if node.val == val => {
                    *cur = node.next.take()
                },
                Some(node) => { 
                    cur = &mut node.next
                },
                None => { break }
            }
        }
        
        head

    }
}






////////////////////////// AI ////////////////////////////////


impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head?; // Early exit if head is None

        // Take ownership of the next pointer instead of cloning it
        head.next = Self::remove_elements(head.next.take(), val);

        if head.val == val {
            head.next // Return the rest of the list, dropping `head`
        } else {
            Some(head)
        }
    }
}