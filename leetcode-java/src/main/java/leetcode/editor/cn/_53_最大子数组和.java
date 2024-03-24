package leetcode.editor.cn;
//给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。 
//
// 子数组 是数组中的一个连续部分。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
//输出：6
//解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
// 
//
// 示例 2： 
//
// 
//输入：nums = [1]
//输出：1
// 
//
// 示例 3： 
//
// 
//输入：nums = [5,4,-1,7,8]
//输出：23
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 10⁵ 
// -10⁴ <= nums[i] <= 10⁴ 
// 
//
// 
//
// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。 
//
// Related Topics 数组 分治 动态规划 👍 6595 👎 0


import java.util.Objects;
import java.util.function.BiFunction;

//leetcode submit region begin(Prohibit modification and deletion)
public class _53_最大子数组和 {
    public int maxSubArray(int[] nums) {
        //return this.dp(nums);

        //return this.sum_of_prefix(nums);

        return this.divideAndConquer(nums);
    }

    /**
     * 时间复杂度：O(n)
     * 空间复杂度：O(1)
     */
    int dp(int[] nums) {
        int pre = 0, res = nums[0];

        for (int num : nums) {
            pre = Math.max(pre + num, num);
            res = Math.max(res, pre);
        }

        return res;
    }

    /**
     * 时间复杂度：O(n)
     * 空间复杂度：O(1)
     */
    int sum_of_prefix(int[] nums) {
        int res = Integer.MIN_VALUE;
        int min_pre_sum = 0, pre_sum = 0;

        for (int num : nums) {
            pre_sum += num;
            res = Math.max(res, pre_sum - min_pre_sum);
            min_pre_sum = Math.min(min_pre_sum, pre_sum);
        }

        return res;
    }

    static class Status {
        int lSum; // 表示区间[l,r]内以l为左端点的最大子段和
        int rSum; // 表示区间[l,r]内以r为右端点的最大子段和
        int mSum; // 表示区间[l,r]内的最大子段和
        int iSum; // 表示区间[l,r]内的区间和
    }

    @FunctionalInterface
    interface TriFunction<A, B, C, D> {
        D apply(A a, B b, C c);
    }

    BiFunction<Status, Status, Status> pushUp = (l, r) -> {
        Status s = new Status();
        s.lSum = Math.max(l.lSum, l.iSum + r.lSum);
        s.rSum = Math.max(r.rSum, r.iSum + l.rSum);
        s.mSum = Math.max(Math.max(l.mSum, r.mSum), l.rSum + r.lSum);
        s.iSum = l.iSum + r.iSum;
        return s;
    };
    TriFunction<int[], Integer, Integer, Status> get = (nums, l, r) -> {
        if (Objects.equals(l, r)) {
            Status s = new Status();
            s.lSum = s.rSum = s.mSum = s.iSum = nums[l];
            return s;
        }

        int m = (l + r) >> 1;
        Status lSub = this.get.apply(nums, l, m);
        Status rSub = this.get.apply(nums, m + 1, r);

        return this.pushUp.apply(lSub, rSub);
    };

    /**
     * 时间复杂度：O(n)
     * 空间复杂度：O(log(n))
     */
    int divideAndConquer(int[] nums) {
        return this.get.apply(nums, 0, nums.length - 1).mSum;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
