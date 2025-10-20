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


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public int MySqrt(int x)
    {
        //return LeftCloseRightOpen(x);
        return LeftCloseRightClose(x);
    }

    int LeftCloseRightOpen(int x)
    {
        long newX = x;
        (long left, long right) = (0, newX + 1);

        while (left < right)
        {
            long mid = left + (right - left) / 2;
            long square = mid * mid;
            if (square > newX)
                right = mid;
            else if (square < newX)
                left = mid + 1;
            else
                return (int)mid;
        }

        return (int)left - 1;
    }
    
    int LeftCloseRightClose(int x)
    {
        long newX = x;
        (long left, long right) = (0, newX);

        while (left <= right)
        {
            long mid = left + (right - left) / 2;
            long square = mid * mid;
            if (square > newX)
                right = mid - 1;
            else if (square < newX)
                left = mid + 1;
            else
                return (int)mid;
        }

        return (int)left - 1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
