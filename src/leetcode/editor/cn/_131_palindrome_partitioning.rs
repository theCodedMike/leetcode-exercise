//Given a string s, partition s such that every substring of the partition is a
//palindrome. Return all possible palindrome partitioning of s.
//
//
// Example 1:
// Input: s = "aab"
//Output: [["a","a","b"],["aa","b"]]
//
// Example 2:
// Input: s = "a"
//Output: [["a"]]
//
//
// Constraints:
//
//
// 1 <= s.length <= 16
// s contains only lowercase English letters.
//
//
// Related Topics String Dynamic Programming Backtracking 👍 11596 👎 369

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        Self::backtracking(s)

        //Self::backtracking_and_dp(s)
    }

    fn backtracking(s: String) -> Vec<Vec<String>> {
        const IS_PALINDROME: fn(&str) -> bool = |s| {
            let mut is_palindrome = true;
            let (mut i, mut j) = (0, s.len() - 1);

            while i < j {
                if &s[i..i + 1] != &s[j..j + 1] {
                    is_palindrome = false;
                }
                (i, j) = (i + 1, j - 1)
            }

            is_palindrome
        };
        const DFS: for<'a> fn(usize, &'a str, &mut Vec<&'a str>, &mut Vec<Vec<String>>) =
            |i, s, combine, res| {
                if i == s.len() {
                    res.push(combine.iter().map(|&s| s.to_string()).collect::<Vec<_>>());
                    return;
                }

                for j in (i + 1)..=s.len() {
                    let substring = &s[i..j];
                    if IS_PALINDROME(substring) {
                        combine.push(substring);
                        DFS(j, s, combine, res);
                        combine.pop();
                    }
                }
            };
        let mut res = vec![];

        DFS(0, &s, &mut vec![], &mut res);

        res
    }

    fn backtracking_and_dp(s: String) -> Vec<Vec<String>> {
        let len = s.len();
        let mut f = vec![vec![true; len]; len];
        for i in (0..len).rev() {
            for j in i + 1..len {
                f[i][j] = (&s[i..i + 1] == &s[j..j + 1]) && f[i + 1][j - 1];
            }
        }

        const DFS: fn(usize, &str, &mut Vec<String>, &mut Vec<Vec<String>>, &Vec<Vec<bool>>) =
            |i, s, combine, res, f| {
                if i == s.len() {
                    res.push(combine.clone());
                    return;
                }

                for j in i..s.len() {
                    if f[i][j] {
                        combine.push(s[i..j + 1].to_string());
                        DFS(j + 1, s, combine, res, f);
                        combine.pop();
                    }
                }
            };
        let mut res = vec![];

        DFS(0, &s, &mut vec![], &mut res, &f);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
