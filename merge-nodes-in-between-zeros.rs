// https://leetcode.com/problems/merge-nodes-in-between-zeros/

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
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;

        let mut sum = 0;
        let mut current_node = &head;
        let mut cur_result_node = &mut result;
        while let Some(node) = current_node{
            sum += node.val;
            if let Some(next) = node.next.as_ref() {
                if next.val == 0 {
                    let new = Some(Box::new(ListNode::new(sum)));
                    if let Some(ref mut cur) = cur_result_node {
                        cur.next = new;
                        cur_result_node = &mut cur.next;
                    } else {
                        result = new;
                        cur_result_node = &mut result;
                    }
                    sum = 0;
                }
            }
            current_node = &node.next;
        }
        result
    }
}
