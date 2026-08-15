//! Grind 169 — LeetCode #721 Accounts Merge (Medium)
//!
//! Given a list of accounts, each with a name and a list of emails,
//! merge accounts that share at least one email (same person), returning
//! merged accounts with sorted emails. Solved with a disjoint-set union
//! over account indices: any two accounts sharing an email get unioned,
//! then all emails owned by each connected component are grouped
//! together.
//!
//! Example:
//!   Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],
//!     ["John","johnsmith@mail.com","john00@mail.com"],
//!     ["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
//!   Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],
//!            ["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]

use std::collections::HashMap;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.parent[ra] = rb;
        }
    }
}

struct Solution;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = accounts.len();
        let mut dsu = Dsu::new(n);
        let mut email_owner: HashMap<String, usize> = HashMap::new();

        for (i, acc) in accounts.iter().enumerate() {
            for email in &acc[1..] {
                if let Some(&owner) = email_owner.get(email) {
                    dsu.union(i, owner);
                } else {
                    email_owner.insert(email.clone(), i);
                }
            }
        }

        let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
        for (email, &id) in &email_owner {
            let root = dsu.find(id);
            groups.entry(root).or_insert_with(Vec::new).push(email.clone());
        }

        let mut result = Vec::new();
        for (root, mut emails) in groups {
            emails.sort();
            let mut entry = vec![accounts[root][0].clone()];
            entry.extend(emails);
            result.push(entry);
        }
        result
    }
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn normalize(mut accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    accounts.sort();
    accounts
}

fn main() {
    let accounts = vec![
        v(&["John", "johnsmith@mail.com", "john_newyork@mail.com"]),
        v(&["John", "johnsmith@mail.com", "john00@mail.com"]),
        v(&["Mary", "mary@mail.com"]),
        v(&["John", "johnnybravo@mail.com"]),
    ];
    println!("{:?}", normalize(Solution::accounts_merge(accounts)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let accounts = vec![
            v(&["John", "johnsmith@mail.com", "john_newyork@mail.com"]),
            v(&["John", "johnsmith@mail.com", "john00@mail.com"]),
            v(&["Mary", "mary@mail.com"]),
            v(&["John", "johnnybravo@mail.com"]),
        ];
        let result = normalize(Solution::accounts_merge(accounts));
        let expected = normalize(vec![
            v(&[
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ]),
            v(&["Mary", "mary@mail.com"]),
            v(&["John", "johnnybravo@mail.com"]),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn no_shared_emails_stay_separate() {
        let accounts = vec![v(&["A", "a@mail.com"]), v(&["B", "b@mail.com"])];
        let result = normalize(Solution::accounts_merge(accounts));
        let expected = normalize(vec![v(&["A", "a@mail.com"]), v(&["B", "b@mail.com"])]);
        assert_eq!(result, expected);
    }

    #[test]
    fn single_account() {
        let accounts = vec![v(&["A", "a@mail.com", "a2@mail.com"])];
        let result = Solution::accounts_merge(accounts);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0][0], "A");
    }
}
