//假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
//
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
//
//
//
// 示例 1：
//
//
//输入：n = 2
//输出：2
//解释：有两种方法可以爬到楼顶。
//1. 1 阶 + 1 阶
//2. 2 阶
//
// 示例 2：
//
//
//输入：n = 3
//输出：3
//解释：有三种方法可以爬到楼顶。
//1. 1 阶 + 1 阶 + 1 阶
//2. 1 阶 + 2 阶
//3. 2 阶 + 1 阶
//
//
//
//
// 提示：
//
//
// 1 <= n <= 45
//
//
// Related Topics 记忆化搜索 数学 动态规划 👍 3481 👎 0

package src

import "math"

// leetcode submit region begin(Prohibit modification and deletion)
func climbStairs(n int) int {
	//return dp(n)

	//return matrixFastPower(n)

	return generalFormula(n)
}

func dp(n int) int {
	p, q, r := 0, 0, 1

	for range n {
		p = q
		q = r
		r = p + q
	}

	return r
}

func matrixFastPower(n int) int {
	m := make([][]int, 2)
	m[0], m[1] = []int{1, 1}, []int{1, 0}

	multiply := func(a, b [][]int) [][]int {
		c := make([][]int, 2)
		c[0], c[1] = []int{0, 0}, []int{0, 0}

		for i := range 2 {
			for j := range 2 {
				c[i][j] = a[i][0]*b[0][j] + a[i][1]*b[1][j]
			}
		}

		return c
	}
	pow := func() [][]int {
		ret := make([][]int, 2)
		ret[0], ret[1] = []int{1, 0}, []int{0, 1}

		for n > 0 {
			if n&1 == 1 {
				ret = multiply(ret, m)
			}
			n >>= 1
			m = multiply(m, m)
		}

		return ret
	}

	res := pow()
	return res[0][0]
}

func generalFormula(n int) int {
	sqrt5 := math.Sqrt(5)
	fibN := math.Pow((1+sqrt5)/2, float64(n+1)) - math.Pow((1-sqrt5)/2, float64(n+1))

	return int(math.Round(fibN / sqrt5))
}

//leetcode submit region end(Prohibit modification and deletion)
