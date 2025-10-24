//给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
//
// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
//
//
//
// 示例 1:
//
//
//输入: nums = [0,1,0,3,12]
//输出: [1,3,12,0,0]
//
//
// 示例 2:
//
//
//输入: nums = [0]
//输出: [0]
//
//
//
// 提示:
//
//
//
// 1 <= nums.length <= 10⁴
// -2³¹ <= nums[i] <= 2³¹ - 1
//
//
//
//
// 进阶：你能尽量减少完成的操作次数吗？
//
// Related Topics 数组 双指针 👍 2750 👎 0

package src

// leetcode submit region begin(Prohibit modification and deletion)
func moveZeroes(nums []int) {
	//insertion(nums)
	twoPointers(nums)
}

func insertion(nums []int) {
	for i := 1; i < len(nums); i++ {
		if nums[i] != 0 {
			j := i
			for j > 0 && nums[j-1] == 0 {
				j--
			}
			nums[j], nums[i] = nums[i], nums[j]
		}
	}
}

func twoPointers(nums []int) {
	slow := 0

	for fast := 0; fast < len(nums); fast++ {
		if nums[fast] != 0 {
			nums[slow], nums[fast] = nums[fast], nums[slow]
			slow++
		}
	}
}

//leetcode submit region end(Prohibit modification and deletion)
