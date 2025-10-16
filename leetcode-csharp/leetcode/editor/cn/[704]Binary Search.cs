//Given an array of integers nums which is sorted in ascending order, and an 
//integer target, write a function to search target in nums. If target exists, then 
//return its index. Otherwise, return -1. 
//
// You must write an algorithm with O(log n) runtime complexity. 
//
// 
// Example 1: 
//
// 
//Input: nums = [-1,0,3,5,9,12], target = 9
//Output: 4
//Explanation: 9 exists in nums and its index is 4
// 
//
// Example 2: 
//
// 
//Input: nums = [-1,0,3,5,9,12], target = 2
//Output: -1
//Explanation: 2 does not exist in nums so return -1
// 
//
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 10⁴ 
// -10⁴ < nums[i], target < 10⁴ 
// All the integers in nums are unique. 
// nums is sorted in ascending order. 
// 
//
// Related Topics 数组 二分查找 👍 1801 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public int Search(int[] nums, int target)
    {
        //return FullMatch(nums, target);
        //return MatchRight(nums, target);
        //return MatchLeft(nums, target);
        return UseStd(nums, target);
    }

    private int FullMatch(int[] nums, int target)
    {
        int left = 0, right = nums.Length;

        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (target < nums[mid])
                right = mid;
            else if (nums[mid] < target)
                left = mid + 1;
            else
                return mid;
        }

        return -1;
    }
    
    private int MatchRight(int[] nums, int target)
    {
        int left = 0, right = nums.Length;

        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (target >= nums[mid])
                left = mid + 1;
            else
                right = mid;
        }

        if (left > 0 && nums[left - 1] == target)
            return left - 1;
        return -1;
    }
    
    private int MatchLeft(int[] nums, int target)
    {
        int left = 0, right = nums.Length;

        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (target <= nums[mid])
                right = mid;
            else
                left = mid + 1;
        }

        if (left < nums.Length && nums[left] == target)
            return left;
        return -1;
    }
    
    private int UseStd(int[] nums, int target)
    {
        int idx = Array.BinarySearch(nums, target);
        return idx >= 0 ? idx : -1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
