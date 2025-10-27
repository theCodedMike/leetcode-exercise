//给定一个含有 n 个正整数的数组和一个正整数 target 。
//
// 找出该数组中满足其总和大于等于 target 的长度最小的 子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其
//长度。如果不存在符合条件的子数组，返回 0 。
//
//
//
// 示例 1：
//
//
//输入：target = 7, nums = [2,3,1,2,4,3]
//输出：2
//解释：子数组 [4,3] 是该条件下的长度最小的子数组。
//
//
// 示例 2：
//
//
//输入：target = 4, nums = [1,4,4]
//输出：1
//
//
// 示例 3：
//
//
//输入：target = 11, nums = [1,1,1,1,1,1,1,1]
//输出：0
//
//
//
//
// 提示：
//
//
// 1 <= target <= 10⁹
// 1 <= nums.length <= 10⁵
// 1 <= nums[i] <= 10⁴
//
//
//
//
// 进阶：
//
//
// 如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。
//
//
// Related Topics 数组 二分查找 前缀和 滑动窗口 👍 2538 👎 0

package src

import (
	"math"
	"slices"
)

// leetcode submit region begin(Prohibit modification and deletion)
func minSubArrayLen(target int, nums []int) int {
	//return bruteForce(target, nums)
	//return binarySearch(target, nums)
	return slidingWindow(target, nums)
}

func bruteForce(target int, nums []int) int {
	size := len(nums)

	for width := 1; width <= size; width++ {
		beg, end := 0, width

		for end <= size {
			sum := 0
			for i := beg; i < end; i++ {
				sum += nums[i]
			}
			if sum >= target {
				return width
			}
			beg++
			end = beg + width
		}
	}

	return 0
}

func binarySearch(target int, nums []int) int {
	size := len(nums)
	res := math.MaxInt
	sums := make([]int, size+1)

	for i := 0; i < size; i++ {
		sums[i+1] = sums[i] + nums[i]
	}

	for i := 0; i < size; i++ {
		toFind := target + sums[i]
		idx, _ := slices.BinarySearch(sums, toFind)
		if idx <= size {
			res = min(res, idx-i)
		}
	}

	if res != math.MaxInt {
		return res
	}
	return 0
}

func slidingWindow(target int, nums []int) int {
	size := len(nums)
	res := math.MaxInt
	left := 0
	sum := 0

	for i := 0; i < size; i++ {
		sum += nums[i]
		for sum >= target {
			res = min(res, i-left+1)
			sum -= nums[left]
			left++
		}
	}

	if res != math.MaxInt {
		return res
	}
	return 0
}

//leetcode submit region end(Prohibit modification and deletion)
