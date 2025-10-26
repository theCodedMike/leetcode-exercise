//给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
//
//
//
//
//
//
// 示例 1：
//
//
//输入：nums = [-4,-1,0,3,10]
//输出：[0,1,9,16,100]
//解释：平方后，数组变为 [16,1,0,9,100]
//排序后，数组变为 [0,1,9,16,100]
//
// 示例 2：
//
//
//输入：nums = [-7,-3,2,3,11]
//输出：[4,9,9,49,121]
//
//
//
//
// 提示：
//
//
// 1 <= nums.length <= 10⁴
// -10⁴ <= nums[i] <= 10⁴
// nums 已按 非递减顺序 排序
//
//
//
//
// 进阶：
//
//
// 请你设计时间复杂度为 O(n) 的算法解决本问题
//
//
// Related Topics 数组 双指针 排序 👍 1136 👎 0

package src

import "slices"

// leetcode submit region begin(Prohibit modification and deletion)
func sortedSquares(nums []int) []int {
	//return bruteForce(nums)
	//return twoPointers1(nums)
	return twoPointers2(nums)
}

// / Time Complexity: O(n log(n))
// /
// / Space Complexity: O(log(n))
func bruteForce(nums []int) []int {
	for i, num := range nums {
		nums[i] = num * num
	}
	slices.Sort(nums)

	return nums
}

// / Time Complexity: O(n)
// /
// / Space Complexity: O(n)
func twoPointers1(nums []int) []int {
	res, idx := make([]int, len(nums)), len(nums)-1
	left, right := 0, len(nums)-1

	for left <= right {
		leftSquare := nums[left] * nums[left]
		rightSquare := nums[right] * nums[right]
		if leftSquare > rightSquare {
			res[idx] = leftSquare
			left++
			idx--
		} else if leftSquare < rightSquare {
			res[idx] = rightSquare
			right--
			idx--
		} else {
			res[idx] = rightSquare
			if left != right {
				res[idx-1] = leftSquare
			}
			left++
			right--
			idx -= 2
		}
	}

	return res
}

// / Time Complexity: O(n)
// /
// / Space Complexity: O(n)
func twoPointers2(nums []int) []int {
	res, idx := make([]int, len(nums)), len(nums)-1
	left, right := 0, len(nums)-1

	for left <= right {
		leftSquare := nums[left] * nums[left]
		rightSquare := nums[right] * nums[right]
		if leftSquare > rightSquare {
			res[idx] = leftSquare
			left++
			idx--
		} else {
			res[idx] = rightSquare
			right--
			idx--
		}
	}

	return res
}

//leetcode submit region end(Prohibit modification and deletion)
