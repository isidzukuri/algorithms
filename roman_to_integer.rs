// https://leetcode.com/problems/roman-to-integer/description/

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {

        let mut vals = HashMap::new();
        vals.insert(String::from("I"), 1);
        vals.insert(String::from("V"), 5);
        vals.insert(String::from("X"), 10);
        vals.insert(String::from("L"), 50);
        vals.insert(String::from("C"), 100);
        vals.insert(String::from("D"), 500);
        vals.insert(String::from("M"), 1000);

        let mut i = s.len();
        let mut sum = 0;

        while i > 0 {
            i -=1;

            let cur_l = s.chars().nth(i).unwrap();
            let cur_val = vals.get(&cur_l.to_string()).unwrap();

            if i > 0 {
                let prev_l = s.chars().nth(i-1).unwrap();
                let prev_val = vals.get(&prev_l.to_string()).unwrap();
                sum += cur_val;
                if prev_val < cur_val{
                    sum -= prev_val;
                    i -=1;
                }
            } else {
                sum += cur_val;
            }
        }
        sum
    }
}
