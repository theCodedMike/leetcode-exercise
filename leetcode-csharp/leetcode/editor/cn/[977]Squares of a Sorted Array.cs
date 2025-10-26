//Given an integer array nums sorted in non-decreasing order, return an array 
//of the squares of each number sorted in non-decreasing order. 
//
// 
// Example 1: 
//
// 
//Input: nums = [-4,-1,0,3,10]
//Output: [0,1,9,16,100]
//Explanation: After squaring, the array becomes [16,1,0,9,100].
//After sorting, it becomes [0,1,9,16,100].
// 
//
// Example 2: 
//
// 
//Input: nums = [-7,-3,2,3,11]
//Output: [4,9,9,49,121]
// 
//
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 10⁴ 
// -10⁴ <= nums[i] <= 10⁴ 
// nums is sorted in non-decreasing order. 
// 
//
// 
//Follow up: Squaring each element and sorting the new array is very trivial, 
//could you find an 
//O(n) solution using a different approach?
//
// Related Topics 数组 双指针 排序 👍 1136 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public int[] SortedSquares(int[] nums)
    {
        //return BruteForce(nums);
        //return TwoPointers1(nums);
        return TwoPointers2(nums);
    }
    
    int[] BruteForce(int[] nums)
    {
        for (int i = 0; i < nums.Length; i++)
            nums[i] *= nums[i];
        Array.Sort(nums);

        return nums;
    }
    
    int[] TwoPointers1(int[] nums)
    {
        (int[] res, int idx) = (new int[nums.Length], nums.Length - 1);
        (int left, int right) = (0, nums.Length - 1);

        while (left <= right)
        {
            int leftSquare = nums[left] * nums[left];
            int rightSquare = nums[right] * nums[right];
            if (leftSquare > rightSquare)
            {
                res[idx] = leftSquare;
                left++;
                idx--;
            } else if (leftSquare < rightSquare)
            {
                res[idx] = rightSquare;
                right--;
                idx--;
            }
            else
            {
                res[idx] = rightSquare;
                if (left != right)
                    res[idx - 1] = leftSquare;
                right--;
                left++;
                idx -= 2;
            }
        }

        return res;
    }
    
    int[] TwoPointers2(int[] nums)
    {
        (int[] res, int idx) = (new int[nums.Length], nums.Length - 1);
        (int left, int right) = (0, nums.Length - 1);

        while (left <= right)
        {
            int leftSquare = nums[left] * nums[left];
            int rightSquare = nums[right] * nums[right];
            if (leftSquare > rightSquare)
            {
                res[idx] = leftSquare;
                left++;
                idx--;
            } else
            {
                res[idx] = rightSquare;
                right--;
                idx--;
            }
        }

        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
