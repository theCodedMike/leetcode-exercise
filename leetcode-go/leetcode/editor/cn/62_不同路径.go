//一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。
//
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。
//
// 问总共有多少条不同的路径？
//
//
//
// 示例 1：
//
//
//输入：m = 3, n = 7
//输出：28
//
// 示例 2：
//
//
//输入：m = 3, n = 2
//输出：3
//解释：
//从左上角开始，总共有 3 条路径可以到达右下角。
//1. 向右 -> 向下 -> 向下
//2. 向下 -> 向下 -> 向右
//3. 向下 -> 向右 -> 向下
//
//
// 示例 3：
//
//
//输入：m = 7, n = 3
//输出：28
//
//
// 示例 4：
//
//
//输入：m = 3, n = 3
//输出：6
//
//
//
// 提示：
//
//
// 1 <= m, n <= 100
// 题目数据保证答案小于等于 2 * 10⁹
//
//
// Related Topics 数学 动态规划 组合数学 👍 2018 👎 0

package src

// leetcode submit region begin(Prohibit modification and deletion)
func uniquePaths(m int, n int) int {
	//return dp(m, n)

	//return optimizeDP(m, n)

	return combinatorics(m, n)
}

func dp(m int, n int) int {
	f := make([][]int, m)
	for i := range f {
		f[i] = make([]int, n)
	}

	for i := 0; i < m; i++ {
		f[i][0] = 1
	}
	for i := 0; i < n; i++ {
		f[0][i] = 1
	}
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			f[i][j] = f[i-1][j] + f[i][j-1]
		}
	}

	return f[m-1][n-1]
}

func optimizeDP(m int, n int) int {
	f := make([]int, n)
	for i := 0; i < n; i++ {
		f[i] = 1
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			f[j] += f[j-1]
		}
	}

	return f[n-1]
}

func combinatorics(m int, n int) int {
	res := 1

	for x, y := n, 1; y < m; x, y = x+1, y+1 {
		res = res * x / y
	}

	return res
}

//leetcode submit region end(Prohibit modification and deletion)
