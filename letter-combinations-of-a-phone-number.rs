https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut num_map = HashMap::new();
        num_map.insert("2", vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        num_map.insert("3", vec!["d".to_string(), "e".to_string(), "f".to_string()]);
        num_map.insert("4", vec!["g".to_string(), "h".to_string(), "i".to_string()]);
        num_map.insert("5", vec!["j".to_string(), "k".to_string(), "l".to_string()]);
        num_map.insert("6", vec!["m".to_string(), "n".to_string(), "o".to_string()]);
        num_map.insert("7", vec!["p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()]);
        num_map.insert("8", vec!["t".to_string(), "u".to_string(), "v".to_string()]);
        num_map.insert("9", vec!["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()]);

        let mut result = vec![];

        let mut blocks = VecDeque::new();
        for digit in digits.chars(){
            blocks.push_back(num_map.get(&digit.to_string().as_str()).unwrap().clone());
        }

        if blocks.is_empty() { return result }

        let mut stacks: Vec<VecDeque<String>> = vec![];

        for s in 0..blocks[0].len() {
            stacks.push(VecDeque::new());
        }

        while let Some(base) = blocks.pop_front() {
            let mut new_part = vec![];
            let mut i = 0;
            let next_part = if blocks.get(0).is_some() {
                blocks[0].clone()
            } else{
                vec![]
            };
            for letter in base.iter(){
                if stacks.get(i).is_some() {
                    stacks[i] = next_part.clone().into();
                }

                if stacks.get(i).is_some() && stacks[i].get(0).is_some(){
                    while let Some(next_letter) = stacks[i].pop_front() {
                        new_part.push(format!("{}{}", letter, next_letter));
                    }
                    blocks.pop_front();
                    blocks.push_front(new_part.clone());
                    i = 0;
                } else{
                    result.push(letter.to_string());
                    i+=1;
                }
            }
        }
        result
    }
}
