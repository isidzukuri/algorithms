// https://leetcode.com/problems/valid-anagram/description/

//bitmasking. Represent string as sum of bytes. To avoid math collisions bit masking is used.

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        get_mask(&s) == get_mask(&t)
    }
}

// works properly if one symbol repeats less than 15 times
fn get_mask(st: &str) -> u128 { 
    let mut mask: u128 = 0;
    for byte in st.bytes() {
        // `0x1F` is a mask.
        let pos = byte & 0x1F; 
        // Give each letter a 4-bit counter slot inside the u128
        mask += 1 << (pos * 4); 
    }
    mask
}
