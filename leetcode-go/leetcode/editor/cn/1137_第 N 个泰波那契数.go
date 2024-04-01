//泰波那契序列 Tn 定义如下：
//
// T0 = 0, T1 = 1, T2 = 1, 且在 n >= 0 的条件下 Tn+3 = Tn + Tn+1 + Tn+2
//
// 给你整数 n，请返回第 n 个泰波那契数 Tn 的值。
//
//
//
// 示例 1：
//
// 输入：n = 4
//输出：4
//解释：
//T_3 = 0 + 1 + 1 = 2
//T_4 = 1 + 1 + 2 = 4
//
//
// 示例 2：
//
// 输入：n = 25
//输出：1389537
//
//
//
//
// 提示：
//
//
// 0 <= n <= 37
// 答案保证是一个 32 位整数，即 answer <= 2^31 - 1。
//
//
// Related Topics 记忆化搜索 数学 动态规划 👍 298 👎 0

package src

// leetcode submit region begin(Prohibit modification and deletion)
func tribonacci(n int) int {
	return dpRecur(n)
	//return dpIter(n)

	//return matrixFastPower(n)
}

func dpRecur(n int) int {
	size := n + 1
	if size < 3 {
		size = 3
	}
	cache := make([]int, size)
	for i := range size {
		if i == 0 {
			cache[i] = 0
		} else if i == 1 || i == 2 {
			cache[i] = 1
		} else {
			cache[i] = -1
		}
	}

	var recur func([]int, int) int
	recur = func(cache []int, n int) int {
		if cache[n] != -1 {
			return cache[n]
		}

		sum := recur(cache, n-1) + recur(cache, n-2) + recur(cache, n-3)
		cache[n] = sum

		return sum
	}

	return recur(cache, n)
}

func dpIter(n int) int {
	switch n {
	case 0:
		return 0
	case 1, 2:
		return 1
	default:
		t0, t1, t2, sum := 0, 0, 1, 1

		for i := 2; i < n; i++ {
			t0, t1, t2 = t1, t2, sum
			sum = t0 + t1 + t2
		}

		return sum
	}
}

func matrixFastPower(n int) int {
	if n == 0 {
		return 0
	}
	if n <= 2 {
		return 1
	}

	multiply := func(a, b *[][]int) [][]int {
		c := make([][]int, 3)
		c[0], c[1], c[2] = []int{0, 0, 0}, []int{0, 0, 0}, []int{0, 0, 0}

		for i := range 3 {
			for j := range 3 {
				c[i][j] = (*a)[i][0]*(*b)[0][j] + (*a)[i][1]*(*b)[1][j] + (*a)[i][2]*(*b)[2][j]
			}
		}

		return c
	}
	m := make([][]int, 3)
	m[0], m[1], m[2] = []int{1, 1, 1}, []int{1, 0, 0}, []int{0, 1, 0}
	pow := func() [][]int {
		ret := make([][]int, 3)
		ret[0], ret[1], ret[2] = []int{1, 0, 0}, []int{0, 1, 0}, []int{0, 0, 1}

		for n > 0 {
			if n&1 == 1 {
				ret = multiply(&ret, &m)
			}
			n >>= 1
			m = multiply(&m, &m)
		}

		return ret
	}

	res := pow()
	return res[0][2]
}

//leetcode submit region end(Prohibit modification and deletion)
