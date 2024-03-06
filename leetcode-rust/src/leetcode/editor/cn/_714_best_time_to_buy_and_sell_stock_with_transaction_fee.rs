//You are given an array prices where prices[i] is the price of a given stock
//on the iᵗʰ day, and an integer fee representing a transaction fee.
//
// Find the maximum profit you can achieve. You may complete as many
//transactions as you like, but you need to pay the transaction fee for each transaction.
//
// Note:
//
//
// You may not engage in multiple transactions simultaneously (i.e., you must
//sell the stock before you buy again).
// The transaction fee is only charged once for each stock purchase and sale.
//
//
//
// Example 1:
//
//
//Input: prices = [1,3,2,8,4,9], fee = 2
//Output: 8
//Explanation: The maximum profit can be achieved by:
//- Buying at prices[0] = 1
//- Selling at prices[3] = 8
//- Buying at prices[4] = 4
//- Selling at prices[5] = 9
//The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
//
//
// Example 2:
//
//
//Input: prices = [1,3,7,5,10,3], fee = 3
//Output: 6
//
//
//
// Constraints:
//
//
// 1 <= prices.length <= 5 * 10⁴
// 1 <= prices[i] < 5 * 10⁴
// 0 <= fee < 5 * 10⁴
//
//
// Related Topics 贪心 数组 动态规划 👍 1050 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        //Self::greedy(prices, fee)
        Self::dp(prices, fee)
        //Self::optimized_dp(prices, fee)
    }

    fn greedy(prices: Vec<i32>, fee: i32) -> i32 {
        let mut profit = 0;
        let mut buy = prices[0] + fee;

        for i in 1..prices.len() {
            if prices[i] + fee < buy {
                buy = prices[i] + fee;
            } else if prices[i] > buy {
                profit += prices[i] - buy;
                buy = prices[i];
            }
        }

        profit
    }

    fn dp(prices: Vec<i32>, fee: i32) -> i32 {
        let len = prices.len();
        let mut dp = vec![[0; 2]; len];
        (dp[0][0], dp[0][1]) = (0, -prices[0]);

        for i in 1..len {
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i] - fee);
            dp[i][1] = std::cmp::max(dp[i - 1][1], dp[i - 1][0] - prices[i]);
        }

        dp[len - 1][0]
    }

    fn optimized_dp(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut sell, mut buy) = (0, -prices[0]);

        for i in 1..prices.len() {
            (sell, buy) = (
                std::cmp::max(sell, buy + prices[i] - fee),
                std::cmp::max(buy, sell - prices[i]),
            );
        }

        sell
    }
}
//leetcode submit region end(Prohibit modification and deletion)
