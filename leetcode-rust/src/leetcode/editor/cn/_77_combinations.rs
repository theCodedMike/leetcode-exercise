//Given two integers n and k, return all possible combinations of k numbers
//chosen from the range [1, n].
//
// You may return the answer in any order.
//
//
// Example 1:
//
//
//Input: n = 4, k = 2
//Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
//Explanation: There are 4 choose 2 = 6 total combinations.
//Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to
//be the same combination.
//
//
// Example 2:
//
//
//Input: n = 1, k = 1
//Output: [[1]]
//Explanation: There is 1 choose 1 = 1 total combination.
//
//
//
// Constraints:
//
//
// 1 <= n <= 20
// 1 <= k <= n
//
//
// Related Topics Backtracking 👍 7492 👎 205

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::backtracking(n, k)

        //Self::combination_enum_recur(n, k)

        //Self::combination_enum_iter(n, k as usize)
    }

    fn backtracking(n: i32, k: i32) -> Vec<Vec<i32>> {
        const BACKTRACKING: fn(i32, i32, usize, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |start, n, k, path, res| {
                // prune
                if (n - start + 1) as usize + path.len() < k {
                    return;
                }

                if path.len() == k {
                    res.push(path.clone());
                    return;
                }

                for i in start..=n {
                    path.push(i);
                    BACKTRACKING(i + 1, n, k, path, res);
                    path.pop();
                }
            };
        let mut res = vec![];

        BACKTRACKING(1, n, k as usize, &mut vec![], &mut res);

        res
    }

    fn combination_enum_recur(n: i32, k: i32) -> Vec<Vec<i32>> {
        const DFS: fn(i32, i32, usize, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |start, n, k, path, res| {
                // prune
                if (n - start + 1) as usize + path.len() < k {
                    return;
                }

                if path.len() == k {
                    res.push(path.clone());
                    return;
                }

                path.push(start);
                DFS(start + 1, n, k, path, res);
                path.pop();
                DFS(start + 1, n, k, path, res);
            };
        let mut res = vec![];

        DFS(1, n, k as usize, &mut vec![], &mut res);

        res
    }

    fn combination_enum_iter(n: i32, k: usize) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = (1..=k)
            .into_iter()
            .fold(Vec::with_capacity(k + 1), |mut path, val| {
                path.push(val as i32);
                path
            });
        path.push(n + 1);

        let mut j = 0;
        while j < k {
            res.push(path[..k].to_vec());
            j = 0;

            while j < k && path[j] + 1 == path[j + 1] {
                path[j] = j as i32 + 1;
                j += 1;
            }

            path[j] += 1;
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
