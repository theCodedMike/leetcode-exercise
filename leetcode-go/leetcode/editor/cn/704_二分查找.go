//给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target ，写一个函数搜索 nums 中的 target，如果 target 存在返
//回下标，否则返回 -1。
//
// 你必须编写一个具有 O(log n) 时间复杂度的算法。
//
// 示例 1:
//
//
//输入: nums = [-1,0,3,5,9,12], target = 9
//输出: 4
//解释: 9 出现在 nums 中并且下标为 4
//
//
// 示例 2:
//
//
//输入: nums = [-1,0,3,5,9,12], target = 2
//输出: -1
//解释: 2 不存在 nums 中因此返回 -1
//
//
//
//
// 提示：
//
//
// 你可以假设 nums 中的所有元素是不重复的。
// n 将在 [1, 10000]之间。
// nums 的每个元素都将在 [-9999, 9999]之间。
//
//
// Related Topics 数组 二分查找 👍 1800 👎 0

package src

import "slices"

// leetcode submit region begin(Prohibit modification and deletion)
func search(nums []int, target int) int {
	//return fullMatch(nums, target)
	//return matchRight(nums, target)
	//return matchLeft(nums, target)
	return useStd(nums, target)
}

func fullMatch(nums []int, target int) int {
	left, right := 0, len(nums)

	for left < right {
		mid := left + (right-left)/2
		if target < nums[mid] {
			right = mid
		} else if nums[mid] < target {
			left = mid + 1
		} else {
			return mid
		}
	}

	return -1
}

func matchRight(nums []int, target int) int {
	left, right := 0, len(nums)

	for left < right {
		mid := left + (right-left)/2
		if nums[mid] <= target {
			left = mid + 1
		} else {
			right = mid
		}
	}

	if left > 0 && nums[left-1] == target {
		return left - 1
	} else {
		return -1
	}
}

func matchLeft(nums []int, target int) int {
	left, right := 0, len(nums)

	for left < right {
		mid := left + (right-left)/2
		if target <= nums[mid] {
			right = mid
		} else {
			left = mid + 1
		}
	}

	if left < len(nums) && nums[left] == target {
		return left
	} else {
		return -1
	}
}

func useStd(nums []int, target int) int {
	if idx, find := slices.BinarySearch(nums, target); find {
		return idx;
	} else {
		return -1
	}
}

//leetcode submit region end(Prohibit modification and deletion)
