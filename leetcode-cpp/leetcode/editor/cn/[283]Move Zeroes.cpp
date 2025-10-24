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


#include <vector>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        //onePointer(nums);
        twoPointers(nums);
    }

    void onePointer(vector<int>& nums) {
        for (auto i = 1; i < nums.size(); ++i) {
            if (nums[i] != 0) {
                auto j = i;
                while (j > 0 && nums[j - 1] == 0)
                    --j;
                std::swap(nums[j], nums[i]);
            }
        }
    }

    void twoPointers(vector<int>& nums) {
        auto slow = 0;

        for (auto fast = 0; fast < nums.size(); ++fast) {
            if (nums[fast] != 0) {
                std::swap(nums[slow], nums[fast]);
                ++slow;
            }
        }
    }
};
//leetcode submit region end(Prohibit modification and deletion)

