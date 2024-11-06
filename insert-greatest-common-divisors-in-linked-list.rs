// https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list

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

use std::cmp;

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = head.clone();
        let mut cur = &mut new_list;

        while let Some(ref mut node) = cur {
            if let Some(next) = node.next.as_ref() {
                let mut insert = Box::new(ListNode { val: Self::common_divisor(node.val, next.val), 
                                                     next: node.next.take()});
                node.next = Some(insert);
                cur = &mut node.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        new_list
    }

    fn common_divisor(v1: i32, v2: i32) -> i32 {
        let mut divisor = if v1 > v2 {
            v1
        } else {
            v2
        };

        loop {
            if (v1 % divisor) == 0 && (v2 % divisor) <= 0 { break }
            divisor -= 1;
        }
        divisor
    }
}
