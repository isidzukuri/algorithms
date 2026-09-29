// https://leetcode.com/problems/implement-trie-prefix-tree



use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    children: HashMap<u8, Trie>,
    end: bool
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self { children: HashMap::new(), end: false }
    }
    
    fn insert(&mut self, word: String) {
        let mut current = self;
        for byte in word.bytes() {
            current = current.children.entry(byte).or_insert(Trie::new());
        }
        current.end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut current = self;
        for byte in word.bytes() {
            if let Some(node) = current.children.get(&byte) {
                current = node;
            } else {
                return false;
            };
        }
        current.end 
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut current = self;
        for byte in prefix.bytes() {
            if let Some(node) = current.children.get(&byte) {
                current = node;
            } else {
                return false;
            };
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */




//////////////////////////////////////// V1
use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self { children: HashMap::new(), end: false }
    }
    
    fn insert(&mut self, word: String) {
        let mut current = self;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert(Trie::new());
        }
        current.end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut current = self;
        for ch in word.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return false;
            };
        }
        current.end 
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut current = self;
        for ch in prefix.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return false;
            };
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */



 //////////////////////////////////////////////////// AI 

 use std::collections::HashMap;

#[derive(Debug, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut current = self;
        for ch in word.chars() {
            // entry + or_default avoids creating an unneeded Trie when key exists
            current = current.children.entry(ch).or_default();
        }
        current.end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find_node(&word).map_or(false, |node| node.end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find_node(&prefix).is_some()
    }

    // Extracted helper: Traverses the trie and returns the final node if path exists
    fn find_node(&self, query: &str) -> Option<&Trie> {
        let mut current = self;
        for ch in query.chars() {
            current = current.children.get(&ch)?;
        }
        Some(current)
    }
}