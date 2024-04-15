//给定一个正整数 n ，将其拆分为 k 个 正整数 的和（ k >= 2 ），并使这些整数的乘积最大化。
//
// 返回 你可以获得的最大乘积 。 
//
// 
//
// 示例 1: 
//
// 
//输入: n = 2
//输出: 1
//解释: 2 = 1 + 1, 1 × 1 = 1。 
//
// 示例 2: 
//
// 
//输入: n = 10
//输出: 36
//解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。 
//
// 
//
// 提示: 
//
// 
// 2 <= n <= 58 
// 
//
// Related Topics 数学 动态规划 👍 1368 👎 0

package src

import "math"

//leetcode submit region begin(Prohibit modification and deletion)
func integerBreak(n int) int {
    //return dp(n)

    //return optimizeDP(n)

    return mathOp(n)
}

// 时间复杂度：O(n^2)
// 空间复杂度：O(n)
func dp(n int) int {
    dp := make([]int, n+1)

    for i := 2; i <= n; i++ {
        currMax := 0
        for j := 1; j < i; j++ {
            currMax = max(currMax, max(j*(i-j), j*dp[i-j]))
        }
        dp[i] = currMax
    }

    return dp[n]
}

// 时间复杂度：O(n)
// 空间复杂度：O(n)
func optimizeDP(n int) int {
    if n <= 3 {
        return n - 1
    }

    dp := make([]int, n+1)
    dp[2] = 1

    for i := 3; i <= n; i++ {
        dp[i] = max(
            max(2*(i-2), 2*dp[i-2]),
            max(3*(i-3), 3*dp[i-3]),
        )
    }

    return dp[n]
}

// 时间复杂度：O(1)
// 空间复杂度：O(1)
func mathOp(n int) int {
    if n <= 3 {
        return n - 1
    }

    quotient, remainder := float64(n/3), n%3
    switch remainder {
    case 0:
        return int(math.Pow(3, quotient))
    case 1:
        return int(math.Pow(3, quotient-1) * 4)
    default:
        return int(math.Pow(3, quotient) * 2)
    }
}

//leetcode submit region end(Prohibit modification and deletion)
