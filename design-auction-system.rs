// https://leetcode.com/problems/design-auction-system

use std::collections::{HashMap, BTreeSet};

#[derive(Default)]
struct AuctionSystem {
    items: HashMap<i32, BTreeSet<(i32, i32)>>,
    bids: HashMap<(i32, i32), i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuctionSystem {

    fn new() -> Self {
        Self::default()
    }
    
    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        let mut btree = self.items.entry(item_id).or_insert(BTreeSet::new());

        if let Some(old_amount) = self.bids.get(&(item_id, user_id)) {
            btree.remove(&(*old_amount, user_id));
        }

        self.bids.insert((item_id, user_id), bid_amount);
        btree.insert((bid_amount, user_id));
    }
    
    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        self.add_bid(user_id, item_id, new_amount);
    }
    
    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        let mut btree = self.items.get_mut(&item_id).unwrap();

        if let Some(old_amount) = self.bids.remove(&(item_id, user_id)) {
            btree.remove(&(old_amount, user_id));
        }
    }
    
    fn get_highest_bidder(&mut self, item_id: i32) -> i32 {
        if let Some(mut btree) = self.items.get_mut(&item_id) {
            if let Some(bid) = btree.last() {
                bid.1
            } else {
                -1
            }
        } else { 
            -1 
        }

    }
}

/**
 * Your AuctionSystem object will be instantiated and called as such:
 * let obj = AuctionSystem::new();
 * obj.add_bid(userId, itemId, bidAmount);
 * obj.update_bid(userId, itemId, newAmount);
 * obj.remove_bid(userId, itemId);
 * let ret_4: i32 = obj.get_highest_bidder(itemId);
 */

 