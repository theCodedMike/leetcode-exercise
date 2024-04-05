package leetcode.editor.cn;
//给你一个整数数组 nums ，你可以对它进行一些操作。 
//
// 每次操作中，选择任意一个 nums[i] ，删除它并获得 nums[i] 的点数。之后，你必须删除 所有 等于 nums[i] - 1 和 nums[i]
// + 1 的元素。 
//
// 开始你拥有 0 个点数。返回你能通过这些操作获得的最大点数。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [3,4,2]
//输出：6
//解释：
//删除 4 获得 4 个点数，因此 3 也被删除。
//之后，删除 2 获得 2 个点数。总共获得 6 个点数。
// 
//
// 示例 2： 
//
// 
//输入：nums = [2,2,3,3,3,4]
//输出：9
//解释：
//删除 3 获得 3 个点数，接着要删除两个 2 和 4 。
//之后，再次删除 3 获得 3 个点数，再次删除 3 获得 3 个点数。
//总共获得 9 个点数。
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 2 * 10⁴ 
// 1 <= nums[i] <= 10⁴ 
// 
//
// Related Topics 数组 哈希表 动态规划 👍 912 👎 0

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Supplier;

//leetcode submit region begin(Prohibit modification and deletion)
public class _740_删除并获得点数 {
    public int deleteAndEarn(int[] nums) {
        //return this.dp(nums);

        return this.sortingThenDP(nums);
    }

    int dp(int[] nums) {
        int maxVal = 0;
        for (int num : nums) {
            maxVal = Math.max(maxVal, num);
        }

        int[] sum = new int[maxVal + 1];
        for (int num : nums) {
            sum[num] += num;
        }

        Supplier<Integer> rob = () -> {
            int first = sum[0], second = Math.max(sum[0], sum[1]);
            for (int i = 2; i < sum.length; i++) {
                int temp = second;
                second = Math.max(first + sum[i], second);
                first = temp;
            }
            return second;
        };

        return rob.get();
    }

    int sortingThenDP(int[] nums) {
        int res = 0;
        Arrays.sort(nums);

        List<Integer> sum = new ArrayList<>(1) {{
            this.add(nums[0]);
        }};
        Supplier<Integer> rob = () -> {
            int len =  sum.size();
            if (len == 1) {
                return sum.get(0);
            }

            int first = sum.get(0), second = Math.max(sum.get(0), sum.get(1));
            for (int i = 2; i < len; i++) {
                int temp = second;
                second = Math.max(first + sum.get(i), second);
                first = temp;
            }

            return second;
        };

        for (int i = 1; i < nums.length; i++) {
            int val = nums[i];
            if (val == nums[i - 1]) {
                sum.set(sum.size() - 1, sum.getLast() + val);
            } else if (val == nums[i - 1] + 1) {
                sum.add(val);
            } else {
                res += rob.get();
                sum.clear();
                sum.add(val);
            }
        }

        res += rob.get();
        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
