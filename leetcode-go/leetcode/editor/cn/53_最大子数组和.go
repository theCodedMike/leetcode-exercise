//给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
//
// 子数组 是数组中的一个连续部分。
//
//
//
// 示例 1：
//
//
//输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
//输出：6
//解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
//
//
// 示例 2：
//
//
//输入：nums = [1]
//输出：1
//
//
// 示例 3：
//
//
//输入：nums = [5,4,-1,7,8]
//输出：23
//
//
//
//
// 提示：
//
//
// 1 <= nums.length <= 10⁵
// -10⁴ <= nums[i] <= 10⁴
//
//
//
//
// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
//
// Related Topics 数组 分治 动态规划 👍 6595 👎 0

package src

import "math"

// leetcode submit region begin(Prohibit modification and deletion)
func maxSubArray(nums []int) int {
	//return dp(nums)

	//return sumOfPrefix(nums)

	return divideAndConquer(nums)
}

// 时间复杂度：O(n)
// 空间复杂度：O(1)
func dp(nums []int) int {
	pre, res := 0, nums[0]

	for _, num := range nums {
		pre = max(pre+num, num)
		res = max(res, pre)
	}

	return res
}

// 时间复杂度：O(n)
// 空间复杂度：O(1)
func sumOfPrefix(nums []int) int {
	res := math.MinInt
	minPreSum, preSum := 0, 0

	for _, num := range nums {
		preSum += num
		res = max(res, preSum-minPreSum)
		minPreSum = min(minPreSum, preSum)
	}

	return res
}

// 时间复杂度：O(n)
// 空间复杂度：O(log(n))
func divideAndConquer(nums []int) int {
	type Status struct {
		lSum, rSum, mSum, iSum int
	}

	pushUp := func(l Status, r Status) Status {
		return Status{
			max(l.lSum, l.iSum+r.lSum),
			max(r.rSum, r.iSum+l.rSum),
			max(max(l.mSum, r.mSum), l.rSum+r.lSum),
			l.iSum + r.iSum,
		}
	}

	var get func(nums []int, l int, r int) Status
	get = func(nums []int, l int, r int) Status {
		if l == r {
			return Status{
				nums[l], nums[l], nums[l], nums[l],
			}
		}

		m := (l + r) >> 1
		lSub := get(nums, l, m)
		rSub := get(nums, m+1, r)

		return pushUp(lSub, rSub)
	}

	return get(nums, 0, len(nums)-1).mSum
}

//leetcode submit region end(Prohibit modification and deletion)
