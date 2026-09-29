// https://leetcode.com/problems/merge-sorted-array/

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut left_n: usize = 0;
        let mut right_n: usize = 0;  
        let mut m = m;  
    
        loop {
            if right_n == nums2.len() {
                break;
            }
            if left_n >= m as usize {
                nums1[left_n] = nums2[right_n];
                left_n+=1;
                right_n+=1;
            } else if nums1[left_n] >= nums2[right_n] {
                nums1.insert(left_n, nums2[right_n]);
                right_n+=1;
                m +=1;
                nums1.pop();
            } else if nums1[left_n] < nums2[right_n] {
                left_n += 1;
            }
        }
    }
}






// /////////////////////////////////// AI

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m as i32 - 1; // Pointer for the last valid element in nums1
        let mut p2 = n as i32 - 1; // Pointer for the last element in nums2
        let mut p = m + n - 1;     // Pointer for the last position in nums1's total capacity

        while p2 >= 0 {
            if p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            p -= 1;
        }
    }
}