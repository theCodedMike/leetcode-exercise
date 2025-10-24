//Given an integer array nums, move all 0's to the end of it while maintaining 
//the relative order of the non-zero elements. 
//
// Note that you must do this in-place without making a copy of the array. 
//
// 
// Example 1: 
// Input: nums = [0,1,0,3,12]
//Output: [1,3,12,0,0]
// 
// Example 2: 
// Input: nums = [0]
//Output: [0]
// 
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 10⁴ 
// -2³¹ <= nums[i] <= 2³¹ - 1 
// 
//
// 
//Follow up: Could you minimize the total number of operations done?
//
// Related Topics 数组 双指针 👍 2750 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public void MoveZeroes(int[] nums) 
    {
        //OnePointer(nums);
        TwoPointers(nums);
    }
    void OnePointer(int[] nums) 
    {
        for (int i = 1; i < nums.Length; ++i)
        {
            if (nums[i] != 0)
            {
                int j = i;
                while (j > 0 && nums[j - 1] == 0)
                    --j;
                (nums[j], nums[i]) = (nums[i], nums[j]);
            }
        }
    }
    
    void TwoPointers(int[] nums)
    {
        int slow = 0;

        for (int fast = 0; fast < nums.Length; ++fast)
        {
            if (nums[fast] != 0)
            {
                (nums[slow], nums[fast]) = (nums[fast], nums[slow]);
                ++slow;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
