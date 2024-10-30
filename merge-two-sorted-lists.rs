https://leetcode.com/problems/merge-two-sorted-lists/

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match(list1, list2){
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => match left.val <= right.val {
                true => Some(Box::new(ListNode{
                        val: left.val,
                        next: Self::merge_two_lists(left.next, Some(right))
                        })),
                false => Some(Box::new(ListNode{
                        val: right.val,
                        next: Self::merge_two_lists(Some(left), right.next)
                        }))
            }
        }
   }
}
