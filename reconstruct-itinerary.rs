# https://leetcode.com/problems/reconstruct-itinerary

use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut airports = HashMap::new();

        for ticket in tickets {
            airports.entry(ticket[0].clone()).or_insert(vec![]).push(ticket[1].clone())
        }

        for destinations in airports.values_mut() {
            destinations.sort_by(|a, b| b.cmp(a));
        }

        let mut result = vec![];
        let mut stack = vec![String::from("JFK")];

        while let Some(airp) = stack.last() {
            if let Some(destinations) = airports.get_mut(airp) {
                if let Some(dest) = destinations.pop() {
                    stack.push(dest.clone());
                    continue;
                }
            }

            result.push(stack.pop().unwrap());
        }

        result.reverse();
        result
    }
}