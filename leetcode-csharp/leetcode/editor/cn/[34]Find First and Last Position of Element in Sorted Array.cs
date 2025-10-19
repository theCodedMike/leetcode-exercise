//Given an array of integers nums sorted in non-decreasing order, find the 
//starting and ending position of a given target value. 
//
// If target is not found in the array, return [-1, -1]. 
//
// You must write an algorithm with O(log n) runtime complexity. 
//
// 
// Example 1: 
// Input: nums = [5,7,7,8,8,10], target = 8
//Output: [3,4]
// 
// Example 2: 
// Input: nums = [5,7,7,8,8,10], target = 6
//Output: [-1,-1]
// 
// Example 3: 
// Input: nums = [], target = 0
//Output: [-1,-1]
// 
// 
// Constraints: 
//
// 
// 0 <= nums.length <= 10⁵ 
// -10⁹ <= nums[i] <= 10⁹ 
// nums is a non-decreasing array. 
// -10⁹ <= target <= 10⁹ 
// 
//
// Related Topics 数组 二分查找 👍 3125 👎 0


//leetcode submit region begin(Prohibit modification and deletion)

using System.Runtime.Versioning;

public class Solution {
    public int[] SearchRange(int[] nums, int target)
    {
        (int left, int right) = (0, nums.Length);
        int[] res = [-1, -1];

        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (target < nums[mid])
                right = mid;
            else if (nums[mid] < target)
                left = mid + 1;
            else
            {
                (int prev, int next) = (mid, mid);
                (bool movePrev, bool moveNext) = (false, false);

                while (true)
                {
                    movePrev = false;
                    moveNext = false;
                    
                    if (prev != 0 && nums[prev - 1] == target)
                    {
                        movePrev = true;
                        --prev;
                    }
                    if (next != nums.Length - 1 && nums[next + 1] == target)
                    {
                        moveNext = true;
                        ++next;
                    }

                    if (!movePrev && !moveNext)
                        break;
                }

                res[0] = prev;
                res[1] = next;
                break;
            }
        }

        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
