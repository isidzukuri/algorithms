// https://leetcode.com/problems/sort-characters-by-frequency

use std::collections::HashMap;
use std::collections::BinaryHeap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut stats = HashMap::new();
        
        for letter in s.bytes() {
            *stats.entry(letter).or_insert(0) +=1;
        }

        let mut heap: BinaryHeap<(usize, u8)> = stats.into_iter()
                                                     .map(|(byte, count)| (count, byte))
                                                     .collect();

        heap.into_sorted_vec()
            .into_iter()
            .rev()
            .map(|x| (x.1 as char).to_string().repeat(x.0) )
            .collect()
    }
}



////////////////////////////////////// AI

use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut stats = HashMap::new();
        for letter in s.bytes() {
            *stats.entry(letter).or_insert(0) += 1;
        }

        let mut counts: Vec<(u8, usize)> = stats.into_iter().collect();
        // Sort descending by frequency
        counts.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut result = String::with_capacity(s.len());
        for (byte, count) in counts {
            result.push_str(&(byte as char).to_string().repeat(count));
        }

        result
    }
} 