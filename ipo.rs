// https://leetcode.com/problems/ipo/



use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
        projects.sort_unstable_by_key(|&(cpt, _)| cpt);

        let mut founds = w;
        let mut available_projects = BinaryHeap::new();

        let mut projects_idx = 0;
        
        for _ in 0..k {
            while let Some(prj) = projects.get(projects_idx) && founds >= prj.0 {
                available_projects.push(prj.1);
                projects_idx+=1;
            }

            if let Some(profit) = available_projects.pop() {
                founds += profit;
            } else {
                break
            }
        }
        founds
    }
}




use std::collections::HashMap;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut inv_opts = HashMap::new();

        for (idx, invest) in capital.iter().enumerate() {
            let mut opt = inv_opts.entry(invest).or_insert(BinaryHeap::new());
            opt.push(profits[idx]);
        }


        let mut available = k;
        let mut capital = w;

        while available > 0 {
            let mut available_capital = capital;

            let mut max_invest_key = 0;
            let mut max_profit = i32::MIN;

            for (&requirement, projects) in inv_opts.iter().filter(|&(&requirement, _)| requirement <= &available_capital) {
                if let Some(profit_proposition) = projects.peek() {
                    if profit_proposition > &max_profit {
                        max_profit = *profit_proposition;
                        max_invest_key = *requirement;
                    }
                }
            }

            if let Some(mut projects) = inv_opts.get_mut(&max_invest_key) {
                if let Some(profit) = projects.pop() {
                    // println!("available: {}, capital: {}, profit: {}", available, capital, profit);
                    capital += profit;
                }
            }
            available -=1;
        }

        capital
    }
}


////////////////////////////////////////////////// AI

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        
        // Pair up (capital, profit) and sort by capital required ascending
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
        projects.sort_unstable_by_key(|&(c, _)| c);

        // Max-heap to store profits of all projects we can currently afford
        let mut max_profit_heap = BinaryHeap::new();
        let mut i = 0;

        for _ in 0..k {
            // Add all projects that become affordable with our current capital `w`
            while i < n && projects[i].0 <= w {
                max_profit_heap.push(projects[i].1);
                i += 1;
            }

            // If no affordable projects remain in our heap, we cannot proceed
            if let Some(best_profit) = max_profit_heap.pop() {
                w += best_profit;
            } else {
                break;
            }
        }

        w
    }
}