//以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返
//回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。
//
//
//
// 示例 1：
//
//
//输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
//输出：[[1,6],[8,10],[15,18]]
//解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
//
//
// 示例 2：
//
//
//输入：intervals = [[1,4],[4,5]]
//输出：[[1,5]]
//解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
//
//
//
// 提示：
//
//
// 1 <= intervals.length <= 10⁴
// intervals[i].length == 2
// 0 <= starti <= endi <= 10⁴
//
//
// Related Topics 数组 排序 👍 2271 👎 0

package src

import (
	"math"
	"slices"
)

// leetcode submit region begin(Prohibit modification and deletion)
func merge(intervals [][]int) [][]int {
	//return bruteForce(intervals)

	return sorting(intervals)
}

func bruteForce(intervals [][]int) [][]int {
	slices.SortFunc(intervals, func(a, b []int) int {
		return a[0] - b[0]
	})

	start, end := math.MaxInt, math.MinInt
	res := make([][]int, 0)
	for i, size := 0, len(intervals); i < size; i++ {
		start = min(start, intervals[i][0])
		end = max(end, intervals[i][1])

		if (i < size-1 && end < intervals[i+1][0]) || (i == size-1) {
			res = append(res, []int{start, end})
			start, end = math.MaxInt, math.MinInt
		}
	}

	return res
}

func sorting(intervals [][]int) [][]int {
	slices.SortFunc(intervals, func(a, b []int) int {
		return a[0] - b[0]
	})

	res := make([][]int, 0)
	for _, interval := range intervals {
		l, r := interval[0], interval[1]
		size := len(res)

		if size == 0 || res[size-1][1] < l {
			res = append(res, []int{l, r})
		} else {
			res[size-1][1] = max(res[size-1][1], r)
		}
	}

	return res
}

//leetcode submit region end(Prohibit modification and deletion)
