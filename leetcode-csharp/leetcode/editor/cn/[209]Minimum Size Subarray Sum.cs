//Given an array of positive integers nums and a positive integer target, 
//return the minimal length of a subarray whose sum is greater than or equal to target. 
//If there is no such subarray, return 0 instead. 
//
// 
// Example 1: 
//
// 
//Input: target = 7, nums = [2,3,1,2,4,3]
//Output: 2
//Explanation: The subarray [4,3] has the minimal length under the problem 
//constraint.
// 
//
// Example 2: 
//
// 
//Input: target = 4, nums = [1,4,4]
//Output: 1
// 
//
// Example 3: 
//
// 
//Input: target = 11, nums = [1,1,1,1,1,1,1,1]
//Output: 0
// 
//
// 
// Constraints: 
//
// 
// 1 <= target <= 10⁹ 
// 1 <= nums.length <= 10⁵ 
// 1 <= nums[i] <= 10⁴ 
// 
//
// 
//Follow up: If you have figured out the 
//O(n) solution, try coding another solution of which the time complexity is 
//O(n log(n)).
//
// Related Topics 数组 二分查找 前缀和 滑动窗口 👍 2538 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public int MinSubArrayLen(int target, int[] nums)
    {
        //return BruteForce(target, nums);
        //return BinarySearch(target, nums);
        return SlidingWindow(target, nums);
    }
    
    int BruteForce(int target, int[] nums)
    {
        for (int width = 1; width <= nums.Length; width++)
        {
            int beg = 0;
            int end = width;
            while (end <= nums.Length)
            {
                int sum = 0;
                for (int i = beg; i < end; i++)
                    sum += nums[i];
                if (sum >= target)
                    return width;

                beg++;
                end = beg + width;
            }
        }
        
        return 0;
    }
    
    int BinarySearch(int target, int[] nums)
    {
        int res = Int32.MaxValue;
        int[] sums = new int[nums.Length + 1];

        for (int i = 0; i < nums.Length; i++)
            sums[i + 1] = sums[i] + nums[i];

        for (int i = 0; i < nums.Length; i++)
        {
            int toFind = target + sums[i];
            int idx = Array.BinarySearch(sums, toFind);
            if (idx < 0)
                idx = Math.Abs(idx) - 1;
            if (idx <= nums.Length)
                res = Math.Min(res, idx - i);
        }

        return res == Int32.MaxValue ? 0 : res;
    }
    
    int SlidingWindow(int target, int[] nums)
    {
        int res = Int32.MaxValue;
        int left = 0;
        int sum = 0;

        for (int i = 0; i < nums.Length; i++)
        {
            sum += nums[i];
            while (sum >= target)
            {
                res = Math.Min(res, i - left + 1);
                sum -= nums[left];
                ++left;
            }
        }

        return res == Int32.MaxValue ? 0 : res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
