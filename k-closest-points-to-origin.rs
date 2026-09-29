// https://leetcode.com/problems/k-closest-points-to-origin

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut result = vec![];
        for (idx, point) in points.iter().enumerate() {
            heap.push(Reverse((point[0].pow(2) + point[1].pow(2), idx)));
        }

        for _ in 0..k {
            if let Some(Reverse(point)) = heap.pop() {
                result.push(points[point.1].clone());
            }
        }

        result
    }
}