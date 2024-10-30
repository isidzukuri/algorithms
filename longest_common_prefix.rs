https://leetcode.com/problems/longest-common-prefix/description/

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = vec![];
        let mut n = 0;

        'outer: loop {
            let mut ch = None;
            for word in strs.iter() {
                let w_ch = word.chars().nth(n);

                if ch.is_none() { ch = w_ch; }
                if ch != w_ch || ch.is_none() { break 'outer; }
            }
            if ch.is_some() {
                result.push(ch.unwrap().to_string());
                n+=1;
                ch = None;
            } else { break }
        }

        result.join("")
    }
}
