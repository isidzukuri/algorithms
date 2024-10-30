https://leetcode.com/problems/container-with-most-water/

use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = height.len();
        if len < 2 { return result }

        let mut i = 0;
        let mut j = len-1;
        let mut max = 0;
        loop {
            let fst = height[i];
            loop {
                let scd = height[j];

                if scd > max { 
                    max = scd;
                    let x = (j - i) as i32;
                    let y = cmp::min(fst, scd);
                    let volume = x * y;
                    if result < volume { result = volume }
                }

                j -=1;
                if j == i || scd >= fst { 
                    i +=1;
                    break 
                }
            }
            j = len-1;
            max = 0;
            if j == i { break }
        }

        result
    }
}
