    // https://leetcode.com/problems/rotate-array/

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let rotations = (k as usize) % nums.len();
        let mut prefix = vec![0; rotations];
        let mut n = 1;
        for _ in 0..rotations {
            if let Some(val) = nums.pop() {
                prefix[rotations - n] = val;
                n +=1;
            }
        }
        nums.splice(0..0, prefix);
    }
}
