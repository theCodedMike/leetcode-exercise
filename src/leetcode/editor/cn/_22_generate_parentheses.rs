//Given n pairs of parentheses, write a function to generate all combinations
//of well-formed parentheses.
//
//
// Example 1:
// Input: n = 3
//Output: ["((()))","(()())","(())()","()(())","()()()"]
//
// Example 2:
// Input: n = 1
//Output: ["()"]
//
//
// Constraints:
//
//
// 1 <= n <= 8
//
//
// Related Topics String Dynamic Programming Backtracking 👍 18453 👎 744

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // todo!
        let mut res = vec![];
        let s = format!("没有思路:{}", n);
        res.push(s);
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
