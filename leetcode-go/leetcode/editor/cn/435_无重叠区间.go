//给定一个区间的集合 intervals ，其中 intervals[i] = [starti, endi] 。返回 需要移除区间的最小数量，使剩余区间互不重
//叠 。
//
//
//
// 示例 1:
//
//
//输入: intervals = [[1,2],[2,3],[3,4],[1,3]]
//输出: 1
//解释: 移除 [1,3] 后，剩下的区间没有重叠。
//
//
// 示例 2:
//
//
//输入: intervals = [ [1,2], [1,2], [1,2] ]
//输出: 2
//解释: 你需要移除两个 [1,2] 来使剩下的区间没有重叠。
//
//
// 示例 3:
//
//
//输入: intervals = [ [1,2], [2,3] ]
//输出: 0
//解释: 你不需要移除任何区间，因为它们已经是无重叠的了。
//
//
//
//
// 提示:
//
//
// 1 <= intervals.length <= 10⁵
// intervals[i].length == 2
// -5 * 10⁴ <= starti < endi <= 5 * 10⁴
//
//
// Related Topics 贪心 数组 动态规划 排序 👍 1118 👎 0

package src

import "slices"

// leetcode submit region begin(Prohibit modification and deletion)
func eraseOverlapIntervals(intervals [][]int) int {
	//return dp(intervals)

	return greedy(intervals)
}

// 时间复杂度：O(n^2)
// 空间复杂度：O(n)
func dp(intervals [][]int) int {
	slices.SortFunc(intervals, func(a, b []int) int {
		return a[0] - b[0]
	})
	size := len(intervals)
	f := make([]int, size)
	f[0] = 1
	maxElem := 1

	for i := 1; i < size; i++ {
		for j := 0; j < i; j++ {
			if intervals[i][0] >= intervals[j][1] {
				f[i] = max(f[i], f[j]+1)
				if f[i] > maxElem {
					maxElem = f[i]
				}
			}
		}
	}

	return size - maxElem
}

// 时间复杂度：O(n*log(n))
// 空间复杂度：O(log(n))
func greedy(intervals [][]int) int {
	slices.SortFunc(intervals, func(a, b []int) int {
		return a[1] - b[1]
	})
	end := intervals[0][1]
	res := 0

	for i, size := 1, len(intervals); i < size; i++ {
		if end <= intervals[i][0] {
			end = intervals[i][1]
		} else {
			res++
		}
	}

	return res
}

//leetcode submit region end(Prohibit modification and deletion)
