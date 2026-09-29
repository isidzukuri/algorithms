// https://leetcode.com/problems/accounts-merge

use std::collections::HashMap;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_in_nodes = HashMap::new();

        for acc_n in 0..accounts.len() {
            for email_n in 1..accounts[acc_n].len() {
                email_in_nodes.entry(accounts[acc_n][email_n].clone())
                              .or_insert(vec![])
                              .push(acc_n);
            }
        }

        let mut uf = UnionFind::new(accounts.len());

        for connections in email_in_nodes.values() {
            let mut iter = connections.iter();
            if let Some(first) = iter.next() {
                for next in iter {
                    uf.union(*first, *next);
                }
            }
        }

        let mut root_to_emails: HashMap<usize, Vec<String>> = HashMap::new();

        for acc_n in 0..accounts.len() {
            let root_n = uf.find(acc_n); 
            let emails = accounts[acc_n][1..].to_vec();
            
            root_to_emails.entry(root_n)
                          .or_insert(vec![])
                          .extend(emails);
        }

        let mut merged = vec![];
        for (root_n, mut emails) in root_to_emails {
            emails.sort_unstable();
            emails.dedup();

            let mut account = Vec::with_capacity(1 + emails.len());
            account.push(accounts[root_n][0].clone());
            account.extend(emails);
            merged.push(account);
        }

        merged
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    // Create a new universe of n isolated elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    // Find the representative root of element i with path compression
    pub fn find(&mut self, mut i: usize) -> usize {
        // Find the root
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        
        // Path compression: point all visited nodes directly to the root
        let mut curr = i;
        while curr != root {
            let nxt = self.parent[curr];
            self.parent[curr] = root;
            curr = nxt;
        }
        
        root
    }

    // Unify the sets containing element i and element j
    // Returns true if a merge happened, false if they were already in the same set
    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

        // Union by rank: attach smaller depth tree under root of deeper tree
        match self.rank[root_i].cmp(&self.rank[root_j]) {
            std::cmp::Ordering::Less => self.parent[root_i] = root_j,
            std::cmp::Ordering::Greater => self.parent[root_j] = root_i,
            std::cmp::Ordering::Equal => {
                self.parent[root_j] = root_i;
                self.rank[root_i] += 1;
            }
        }
        true
    }
}


/////////////////////////////////////////////////////////////// AI


use std::collections::HashMap;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        let mut curr = i;
        while curr != root {
            let next = self.parent[curr];
            self.parent[curr] = root;
            curr = next;
        }
        root
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_id: HashMap<&str, usize> = HashMap::new();
        let mut id_to_email: Vec<&str> = Vec::new();
        let mut email_to_name: Vec<&str> = Vec::new();

        // 1. Assign unique integer IDs to every unique email
        for acc in &accounts {
            let name = &acc[0];
            for email in acc.iter().skip(1) {
                let id_to_email_ref = &mut id_to_email;
                let email_to_name_ref = &mut email_to_name;

                email_to_id.entry(email.as_str()).or_insert_with(|| {
                    let id = id_to_email_ref.len();
                    id_to_email_ref.push(email.as_str());
                    email_to_name_ref.push(name.as_str());
                    id
                });
            }
        }

        let num_emails = id_to_email.len();
        let mut dsu = Dsu::new(num_emails);

        // 2. Union emails using fast integer operations
        for acc in &accounts {
            if acc.len() <= 1 {
                continue;
            }
            let first_id = email_to_id[acc[1].as_str()];
            for email in acc.iter().skip(2) {
                let current_id = email_to_id[email.as_str()];
                dsu.union(first_id, current_id);
            }
        }

        // 3. Group email IDs by their DSU root representative
        let mut root_to_emails: HashMap<usize, Vec<&str>> = HashMap::new();
        for id in 0..num_emails {
            let root = dsu.find(id);
            root_to_emails
                .entry(root)
                .or_default()
                .push(id_to_email[id]);
        }

        // 4. Construct final sorted accounts
        let mut result = Vec::with_capacity(root_to_emails.len());
        for (root_id, mut emails) in root_to_emails {
            emails.sort_unstable(); // Fast in-memory array sorting

            let mut account = Vec::with_capacity(1 + emails.len());
            account.push(email_to_name[root_id].to_string());
            for email in emails {
                account.push(email.to_string());
            }
            result.push(account);
        }

        result
    }
}