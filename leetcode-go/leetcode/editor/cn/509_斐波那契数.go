//斐波那契数 （通常用 F(n) 表示）形成的序列称为 斐波那契数列 。该数列由 0 和 1 开始，后面的每一项数字都是前面两项数字的和。也就是：
//
//
//F(0) = 0，F(1) = 1
//F(n) = F(n - 1) + F(n - 2)，其中 n > 1
//
//
// 给定 n ，请计算 F(n) 。
//
//
//
// 示例 1：
//
//
//输入：n = 2
//输出：1
//解释：F(2) = F(1) + F(0) = 1 + 0 = 1
//
//
// 示例 2：
//
//
//输入：n = 3
//输出：2
//解释：F(3) = F(2) + F(1) = 1 + 1 = 2
//
//
// 示例 3：
//
//
//输入：n = 4
//输出：3
//解释：F(4) = F(3) + F(2) = 2 + 1 = 3
//
//
//
//
// 提示：
//
//
// 0 <= n <= 30
//
//
// Related Topics 递归 记忆化搜索 数学 动态规划 👍 747 👎 0

package src

import "math"

// leetcode submit region begin(Prohibit modification and deletion)
func fib(n int) int {
	//return recursion(n)

	//return dp(n)

	//return matrixFastPower(n)

	return generalFormula(n)
}

// 时间复杂度：O(n)
// 空间复杂度：O(n)
func recursion(n int) int {
	if n < 2 {
		return n
	}

	s := make([]int, n+1)
	for i := range n + 1 {
		if i < 2 {
			s[i] = i
		} else {
			s[i] = -1
		}
	}

	var recur func(int) int
	recur = func(n int) int {
		if s[n] != -1 {
			return s[n]
		}

		res := recur(n-1) + recur(n-2)
		s[n] = res

		return res
	}

	return recur(n)
}

// 时间复杂度：O(n)
// 空间复杂度：O(1)
func dp(n int) int {
	if n < 2 {
		return n
	}

	prev, curr, sum := 0, 0, 1
	for range n - 1 {
		prev = curr
		curr = sum
		sum = prev + curr
	}

	return sum
}

// 时间复杂度：O(log(n))
// 空间复杂度：O(1)
func matrixFastPower(n int) int {
	if n < 2 {
		return n
	}

	matrixMultiply := func(a, b [][]int) [][]int {
		c := make([][]int, 2)
		c[0], c[1] = []int{0, 0}, []int{0, 0}

		for i := range 2 {
			for j := range 2 {
				c[i][j] = a[i][0]*b[0][j] + a[i][1]*b[1][j]
			}
		}

		return c
	}
	matrixPow := func(a [][]int, n int) [][]int {
		ret := make([][]int, 2)
		ret[0], ret[1] = []int{1, 0}, []int{0, 1}

		for n > 0 {
			if n&1 != 0 {
				ret = matrixMultiply(ret, a)
			}
			n >>= 1
			a = matrixMultiply(a, a)
		}

		return ret
	}

	m := make([][]int, 2)
	m[0], m[1] = []int{1, 1}, []int{1, 0}
	res := matrixPow(m, n-1)

	return res[0][0]
}

// 时间复杂度：O(?)
// 空间复杂度：O(1)
func generalFormula(n int) int {
	sqrt5 := math.Sqrt(5)
	fibN := math.Pow((1+sqrt5)/2, float64(n)) - math.Pow((1-sqrt5)/2, float64(n))

	return int(math.Round(fibN / sqrt5))
}

//leetcode submit region end(Prohibit modification and deletion)
