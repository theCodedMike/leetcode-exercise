package leetcode.editor.cn;
//以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返
//回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。 
//
// 
//
// 示例 1： 
//
// 
//输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
//输出：[[1,6],[8,10],[15,18]]
//解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
// 
//
// 示例 2： 
//
// 
//输入：intervals = [[1,4],[4,5]]
//输出：[[1,5]]
//解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。 
//
// 
//
// 提示： 
//
// 
// 1 <= intervals.length <= 10⁴ 
// intervals[i].length == 2 
// 0 <= starti <= endi <= 10⁴ 
// 
//
// Related Topics 数组 排序 👍 2271 👎 0


import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

//leetcode submit region begin(Prohibit modification and deletion)
public class _56_合并区间 {
    public int[][] merge(int[][] intervals) {
        //return this.bruteForce(intervals);

        return this.sorting(intervals);
    }

    int[][] bruteForce(int[][] intervals) {
        Arrays.sort(intervals, Comparator.comparingInt(a -> a[0]));

        int start = Integer.MAX_VALUE, end = Integer.MIN_VALUE;
        List<int[]> res = new ArrayList<>();
        for (int i = 0, len = intervals.length; i < len; i++) {
            start = Math.min(start, intervals[i][0]);
            end = Math.max(end, intervals[i][1]);

            if ((i < len - 1 && end < intervals[i + 1][0]) || i == len - 1) {
                res.add(new int[]{start, end});
                start = Integer.MAX_VALUE;
                end = Integer.MIN_VALUE;
            }
        }

        return res.toArray(new int[res.size()][]);
    }

    int[][] sorting(int[][] intervals) {
        Arrays.sort(intervals, Comparator.comparingInt(a -> a[0]));

        List<int[]> res = new ArrayList<>();
        for (int[] interval : intervals) {
            int l = interval[0], r = interval[1];

            if (res.isEmpty() || res.getLast()[1] < l) {
                res.add(new int[]{l, r});
            } else {
                res.getLast()[1] = Math.max(res.getLast()[1], r);
            }

        }

        return res.toArray(new int[res.size()][]);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
