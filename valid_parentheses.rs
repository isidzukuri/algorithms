// https://leetcode.com/problems/valid-parentheses/

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut dict = HashMap::new();

        dict.insert(')','(');
        dict.insert(']','[');
        dict.insert('}','{');

        let mut open_stack: Vec<char> = vec![];

        for c in s.chars() {
            let closing = dict.get(&c);
            if closing.is_some() { 
                if closing != open_stack.pop().as_ref() { return false }
            } else {
                open_stack.push(c)
            }
        }
        open_stack.is_empty() 
    }
}
