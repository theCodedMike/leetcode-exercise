package leetcode.editor.cn;
//编写一个算法来判断一个数 n 是不是快乐数。
//
// 「快乐数」 定义为： 
//
// 
// 对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。 
// 然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。 
// 如果这个过程 结果为 1，那么这个数就是快乐数。 
// 
//
// 如果 n 是 快乐数 就返回 true ；不是，则返回 false 。 
//
// 
//
// 示例 1： 
//
// 
//输入：n = 19
//输出：true
//解释：
//1² + 9² = 82
//8² + 2² = 68
//6² + 8² = 100
//1² + 0² + 0² = 1
// 
//
// 示例 2： 
//
// 
//输入：n = 2
//输出：false
// 
//
// 
//
// 提示： 
//
// 
// 1 <= n <= 2³¹ - 1 
// 
//
// Related Topics 哈希表 数学 双指针 👍 1463 👎 0


import java.util.HashSet;
import java.util.Set;

//leetcode submit region begin(Prohibit modification and deletion)
class _202_快乐数 {
    public boolean isHappy(int n) {
        //return this.useHash(n);
        return this.slowFastPointers(n);
    }

    boolean useHash(int n) {
        Set<Integer> set = new HashSet<>();
        while (n != 1 && !set.contains(n)) {
            set.add(n);
            n = this.getNext(n);
        }
        return n == 1;
    }

    int getNext(int n) {
        int sum = 0;
        while (n != 0) {
            int i = n % 10;
            sum += i * i;
            n /= 10;
        }
        return sum;
    }

    boolean slowFastPointers(int n) {
        int slow = n;
        int fast = n;
        do {
            slow = this.getNext(slow);
            fast = this.getNext(this.getNext(fast));
        } while (slow != fast && fast != 1);
        return fast == 1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
