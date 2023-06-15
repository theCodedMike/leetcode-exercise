//You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can
//you climb to the top?
//
//
// Example 1:
//
//
//Input: n = 2
//Output: 2
//Explanation: There are two ways to climb to the top.
//1. 1 step + 1 step
//2. 2 steps
//
//
// Example 2:
//
//
//Input: n = 3
//Output: 3
//Explanation: There are three ways to climb to the top.
//1. 1 step + 1 step + 1 step
//2. 1 step + 2 steps
//3. 2 steps + 1 step
//
//
//
// Constraints:
//
//
// 1 <= n <= 45
//
//
// Related Topics Math Dynamic Programming Memoization 👍 18478 👎 580

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        /// 超时了，后续优化
        /// todo
        count_of_ways(n)
    }
}

fn count_of_ways(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    count_of_ways(n - 1) + count_of_ways(n - 2)
}
//leetcode submit region end(Prohibit modification and deletion)
