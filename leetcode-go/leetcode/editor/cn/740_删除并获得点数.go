//给你一个整数数组 nums ，你可以对它进行一些操作。
//
// 每次操作中，选择任意一个 nums[i] ，删除它并获得 nums[i] 的点数。之后，你必须删除 所有 等于 nums[i] - 1 和 nums[i]
// + 1 的元素。
//
// 开始你拥有 0 个点数。返回你能通过这些操作获得的最大点数。
//
//
//
// 示例 1：
//
//
//输入：nums = [3,4,2]
//输出：6
//解释：
//删除 4 获得 4 个点数，因此 3 也被删除。
//之后，删除 2 获得 2 个点数。总共获得 6 个点数。
//
//
// 示例 2：
//
//
//输入：nums = [2,2,3,3,3,4]
//输出：9
//解释：
//删除 3 获得 3 个点数，接着要删除两个 2 和 4 。
//之后，再次删除 3 获得 3 个点数，再次删除 3 获得 3 个点数。
//总共获得 9 个点数。
//
//
//
//
// 提示：
//
//
// 1 <= nums.length <= 2 * 10⁴
// 1 <= nums[i] <= 10⁴
//
//
// Related Topics 数组 哈希表 动态规划 👍 912 👎 0

package src

import "slices"

// leetcode submit region begin(Prohibit modification and deletion)
func deleteAndEarn(nums []int) int {
	//return dp(nums)

	return sortingThenDP(nums)
}

func dp(nums []int) int {
	var maxVal int
	for _, num := range nums {
		maxVal = max(maxVal, num)
	}

	sum := make([]int, maxVal+1)
	for _, num := range nums {
		sum[num] += num
	}

	rob := func() int {
		first, second := sum[0], max(sum[0], sum[1])
		for i, size := 2, len(sum); i < size; i++ {
			first, second = second, max(first+sum[i], second)
		}
		return second
	}

	return rob()
}

func sortingThenDP(nums []int) int {
	res := 0
	slices.Sort(nums)

	sum := make([]int, 1)
	sum[0] = nums[0]
	rob := func() int {
		size := len(sum)
		if size == 1 {
			return sum[0]
		}

		first, second := sum[0], max(sum[0], sum[1])
		for i := 2; i < size; i++ {
			first, second = second, max(first+sum[i], second)
		}

		return second
	}

	for i, size := 1, len(nums); i < size; i++ {
		val := nums[i]
		if val == nums[i-1] {
			sum[len(sum)-1] += val
		} else if val == nums[i-1]+1 {
			sum = append(sum, val)
		} else {
			res += rob()
			sum = sum[:1]
			sum[0] = val
		}
	}

	res += rob()
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
