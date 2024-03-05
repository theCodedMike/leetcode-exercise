package leetcode.editor.cn;
//给出由小写字母组成的字符串 S，重复项删除操作会选择两个相邻且相同的字母，并删除它们。
//
// 在 S 上反复执行重复项删除操作，直到无法继续删除。 
//
// 在完成所有重复项删除操作后返回最终的字符串。答案保证唯一。 
//
// 
//
// 示例： 
//
// 输入："abbaca"
//输出："ca"
//解释：
//例如，在 "abbaca" 中，我们可以删除 "bb" 由于两字母相邻且相同，这是此时唯一可以执行删除操作的重复项。之后我们得到字符串 "aaca"，其中又
//只有 "aa" 可以执行重复项删除操作，所以最后的字符串为 "ca"。
// 
//
// 
//
// 提示： 
//
// 
// 1 <= S.length <= 20000 
// S 仅由小写英文字母组成。 
// 
//
// Related Topics 栈 字符串 👍 592 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
class _1047_删除字符串中的所有相邻重复项 {
    public String removeDuplicates(String s) {
        return this.bruteForce(s);
        //return this.useStack(s);
    }

    // Time Complexity: O(n^2)
    //
    // Space Complexity: O(n)
    String bruteForce(String s) {
        StringBuilder chars = new StringBuilder(s);

        int curr = 0;
        int next = 1;
        while (next < chars.length()) {
            if (chars.charAt(curr) == chars.charAt(next)) {
                while (true) {
                    if (curr == 0 || next == chars.length() - 1) {
                        break;
                    }
                    if (chars.charAt(curr - 1) == chars.charAt(next + 1)) {
                        curr--;
                        next++;
                    } else {
                        break;
                    }
                }
                chars.delete(curr, next + 1);
                curr = 0;
                next = 1;
            } else {
                curr = next;
                next++;
            }
        }

        return chars.toString();
    }

    // Time Complexity: O(n)
    //
    // Space Complexity: O(n)
    String useStack(String s) {
        StringBuilder stack = new StringBuilder(s.length() / 2);
        int top = -1;

        for (int i = 0, len = s.length(); i < len; i++) {
            char ch = s.charAt(i);
            if (stack.isEmpty() || stack.charAt(top) != ch) {
                stack.append(ch);
                top++;
            } else {
                stack.deleteCharAt(top);
                top--;
            }
        }

        return stack.toString();
    }
}
//leetcode submit region end(Prohibit modification and deletion)
