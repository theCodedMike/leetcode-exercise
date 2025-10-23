//给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持
//一致 。然后返回 nums 中唯一元素的个数。
//
// 考虑 nums 的唯一元素的数量为 k。去重后，返回唯一元素的数量 k。
//
// nums 的前 k 个元素应包含 排序后 的唯一数字。下标 k - 1 之后的剩余元素可以忽略。
//
// 判题标准:
//
// 系统会用下面的代码来测试你的题解:
//
//
//int[] nums = [...]; // 输入数组
//int[] expectedNums = [...]; // 长度正确的期望答案
//
//int k = removeDuplicates(nums); // 调用
//
//assert k == expectedNums.length;
//for (int i = 0; i < k; i++) {
//    assert nums[i] == expectedNums[i];
//}
//
// 如果所有断言都通过，那么您的题解将被 通过。
//
//
//
// 示例 1：
//
//
//输入：nums = [1,1,2]
//输出：2, nums = [1,2,_]
//解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
//
//
// 示例 2：
//
//
//输入：nums = [0,0,1,1,1,2,2,3,3,4]
//输出：5, nums = [0,1,2,3,4,_,_,_,_,_]
//解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。
//
//
//
//
// 提示：
//
//
// 1 <= nums.length <= 3 * 10⁴
// -100 <= nums[i] <= 100
// nums 已按 非递减 顺序排列。
//
//
// Related Topics 数组 双指针 👍 3864 👎 0

package src

import "debug/macho"

// leetcode submit region begin(Prohibit modification and deletion)
func removeDuplicates(nums []int) int {
	//return bruteForce(nums)
	return twoPointers(nums)
}

func bruteForce(nums []int) int {
	size := len(nums)
	target := nums[0]

	for i := 1; i < size; i++ {
		if nums[i] == target {
			for j := i; j < size; j++ {
				nums[j-1] = nums[j]
			}
			size--
			i--
		} else {
			target = nums[i]
		}
	}

	return size
}

func twoPointers(nums []int) int {
	slow := 0

	for fast := 1; fast < len(nums); fast++ {
		if nums[slow] != nums[fast] {
			slow++
			nums[slow] = nums[fast]
		}
	}

	return slow + 1
}

//leetcode submit region end(Prohibit modification and deletion)
