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


#include <algorithm>
#include <vector>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        //return bruteForce(nums);
        //return twoPointers1(nums);
        return twoPointers2(nums);
    }

    vector<int> bruteForce(vector<int>& nums) {
        for (auto i = 0; i < nums.size(); ++i)
            nums[i] *= nums[i];
        std::sort(nums.begin(), nums.end());

        return nums;
    }

    vector<int> twoPointers1(vector<int>& nums) {
        auto[res, idx] = std::make_pair(vector<int>(nums.size()), nums.size() - 1);
        auto[left, right] = std::make_pair(0, static_cast<int>(nums.size()) - 1);

        while (left <= right) {
            auto left_square = nums[left] * nums[left];
            auto right_square = nums[right] * nums[right];
            if (left_square > right_square) {
                res[idx] = left_square;
                left++;
                idx--;
            } else if (left_square < right_square) {
                res[idx] = right_square;
                right--;
                idx--;
            } else {
                res[idx] = right_square;
                if (left != right)
                    res[idx - 1] = left_square;
                left++;
                right--;
                idx -= 2;
            }
        }

        return res;
    }

    vector<int> twoPointers2(vector<int>& nums) {
        auto[res, idx] = std::make_pair(vector<int>(nums.size()), nums.size() - 1);
        auto[left, right] = std::make_pair(0, static_cast<int>(nums.size()) - 1);

        while (left <= right) {
            auto left_square = nums[left] * nums[left];
            auto right_square = nums[right] * nums[right];
            if (left_square > right_square) {
                res[idx] = left_square;
                left++;
                idx--;
            } else {
                res[idx] = right_square;
                right--;
                idx--;
            }
        }

        return res;
    }
};
//leetcode submit region end(Prohibit modification and deletion)

