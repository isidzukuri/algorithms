// https://leetcode.com/problems/palindrome-number/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let max_i = s.len() - 1;
        let mut i = 0;
        while i <= max_i {
            let r_i = max_i - i;
            if i == r_i { break }
            if s.chars().nth(i).unwrap() != s.chars().nth(r_i).unwrap() { return false; }
            i+=1;
        }
        true
    }
}
