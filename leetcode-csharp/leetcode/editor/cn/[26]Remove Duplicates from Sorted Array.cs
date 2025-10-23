//Given an integer array nums sorted in non-decreasing order, remove the 
//duplicates in-place such that each unique element appears only once. The relative 
//order of the elements should be kept the same. 
//
// Consider the number of unique elements in nums to be k. 
// After removing duplicates, return the number of unique elements k. 
//
// 
// The first k elements of nums should contain the unique numbers in sorted 
//order. The remaining elements beyond index k - 1 can be ignored. 
//
// Custom Judge: 
//
// The judge will test your solution with the following code: 
//
// 
//int[] nums = [...]; // Input array
//int[] expectedNums = [...]; // The expected answer with correct length
//
//int k = removeDuplicates(nums); // Calls your implementation
//
//assert k == expectedNums.length;
//for (int i = 0; i < k; i++) {
//    assert nums[i] == expectedNums[i];
//}
// 
//
// If all assertions pass, then your solution will be accepted. 
//
// 
// Example 1: 
//
// 
//Input: nums = [1,1,2]
//Output: 2, nums = [1,2,_]
//Explanation: Your function should return k = 2, with the first two elements 
//of nums being 1 and 2 respectively.
//It does not matter what you leave beyond the returned k (hence they are 
//underscores).
// 
//
// Example 2: 
//
// 
//Input: nums = [0,0,1,1,1,2,2,3,3,4]
//Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
//Explanation: Your function should return k = 5, with the first five elements 
//of nums being 0, 1, 2, 3, and 4 respectively.
//It does not matter what you leave beyond the returned k (hence they are 
//underscores).
// 
//
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 3 * 10⁴ 
// -100 <= nums[i] <= 100 
// nums is sorted in non-decreasing order. 
// 
//
// Related Topics 数组 双指针 👍 3865 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public int RemoveDuplicates(int[] nums)
    {
        //return BruteForce(nums);
        return TwoPointers(nums);
    }
    
    int BruteForce(int[] nums)
    {
        int len = nums.Length;
        int target = nums[0];

        for (int i = 1; i < len; ++i)
        {
            if (nums[i] == target)
            {
                for (int j = i; j < len; ++j)
                    nums[j - 1] = nums[j];
                --len;
                --i;
            }
            else
                target = nums[i];
        }

        return len;
    }
    
    int TwoPointers(int[] nums)
    {
        int slow = 0;

        for (int fast = 1; fast < nums.Length; ++fast)
        {
            if (nums[slow] != nums[fast])
            {
                ++slow;
                nums[slow] = nums[fast];
            }
        }

        return slow + 1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
