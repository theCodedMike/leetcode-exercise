//You are given a list of airline tickets where tickets[i] = [fromi, toi]
//represent the departure and the arrival airports of one flight. Reconstruct the
//itinerary in order and return it.
//
// All of the tickets belong to a man who departs from "JFK", thus, the
//itinerary must begin with "JFK". If there are multiple valid itineraries, you should
//return the itinerary that has the smallest lexical order when read as a single
//string.
//
//
// For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than [
//"JFK", "LGB"].
//
//
// You may assume all tickets form at least one valid itinerary. You must use
//all the tickets once and only once.
//
//
// Example 1:
//
//
//Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
//Output: ["JFK","MUC","LHR","SFO","SJC"]
//
//
// Example 2:
//
//
//Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],[
//"ATL","SFO"]]
//Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
//Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK",
//"ATL","SFO"] but it is larger in lexical order.
//
//
//
// Constraints:
//
//
// 1 <= tickets.length <= 300
// tickets[i].length == 2
// fromi.length == 3
// toi.length == 3
// fromi and toi consist of uppercase English letters.
// fromi != toi
//
//
// Related Topics 深度优先搜索 图 欧拉回路 👍 889 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        //Self::backtracking(tickets)

        Self::hierholzer(tickets)
    }

    fn backtracking(mut tickets: Vec<Vec<String>>) -> Vec<String> {
        const DFS: for<'a> fn(
            &'a Vec<Vec<String>>,
            &mut Vec<bool>,
            &mut Vec<&'a Vec<String>>,
            &mut Vec<String>,
        ) = |tickets, used, path, res| {
            if !res.is_empty() {
                return;
            }

            if path.len() == tickets.len() {
                let len = path.len();
                path.iter().enumerate().for_each(|(i, p)| {
                    res.push(p[0].clone());
                    if i == len - 1 {
                        res.push(p[1].clone());
                    }
                });
                return;
            }

            for i in 0..tickets.len() {
                if used[i] {
                    continue;
                }
                if path.last().is_some_and(|last| last[1] != tickets[i][0]) {
                    continue;
                }
                if path.is_empty() && tickets[i][0] != "JFK" {
                    continue;
                }

                used[i] = true;
                path.push(&tickets[i]);

                DFS(tickets, used, path, res);

                used[i] = false;
                path.pop();
            }
        };
        tickets.sort_unstable();
        let mut used = vec![false; tickets.len()];
        let mut res = Vec::with_capacity(tickets.len());

        DFS(&tickets, &mut used, &mut vec![], &mut res);

        res
    }

    fn hierholzer(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut map = tickets.into_iter().fold(HashMap::new(), |mut map, mut t| {
            let (to, from) = (t.remove(1), t.remove(0));
            match map.get_mut(&from) {
                None => {
                    map.insert(from, BinaryHeap::from([Reverse(to)]));
                }
                Some(heap) => {
                    heap.push(Reverse(to));
                }
            };
            map
        });
        const DFS: fn(String, &mut HashMap<String, BinaryHeap<Reverse<String>>>, &mut Vec<String>) =
            |curr, map, res| {
                while map.contains_key(&curr) && !map[&curr].is_empty() {
                    let next = map
                        .get_mut(&curr)
                        .and_then(|heap| heap.pop())
                        .unwrap_or_default();
                    DFS(next.0, map, res);
                }
                res.push(curr)
            };
        let mut res = vec![];

        DFS("JFK".to_string(), &mut map, &mut res);

        res.reverse();
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
