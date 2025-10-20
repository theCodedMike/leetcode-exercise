//Given a non-negative integer x, return the square root of x rounded down to 
//the nearest integer. The returned integer should be non-negative as well. 
//
// You must not use any built-in exponent function or operator. 
//
// 
// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python. 
// 
//
// 
// Example 1: 
//
// 
//Input: x = 4
//Output: 2
//Explanation: The square root of 4 is 2, so we return 2.
// 
//
// Example 2: 
//
// 
//Input: x = 8
//Output: 2
//Explanation: The square root of 8 is 2.82842..., and since we round it down 
//to the nearest integer, 2 is returned.
// 
//
// 
// Constraints: 
//
// 
// 0 <= x <= 2³¹ - 1 
// 
//
// Related Topics 数学 二分查找 👍 1720 👎 0


#include <iostream>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    int mySqrt(int x) {
        //return leftCloseRightOpen(x);
        return leftCloseRightClose(x);
    }

    int leftCloseRightOpen(int x) {
        const long new_x = x;
        auto[left, right] = std::make_pair(0, new_x + 1);

        while (left < right) {
            const auto mid = left + (right - left) / 2;
            const auto square = mid * mid;
            if (square > new_x)
                right = mid;
            else if (square < new_x)
                left = mid + 1;
            else
                return mid;
        }

        return left - 1;
    }

    int leftCloseRightClose(int x) {
        const long new_x = x;
        auto[left, right] = std::make_pair(0, new_x);

        while (left <= right) {
            const auto mid = left + (right - left) / 2;
            const auto square = mid * mid;
            if (square > new_x)
                right = mid - 1;
            else if (square < new_x)
                left = mid + 1;
            else
                return mid;
        }

        return left - 1;
    }

};
//leetcode submit region end(Prohibit modification and deletion)

