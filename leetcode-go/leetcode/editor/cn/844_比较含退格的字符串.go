//给定 s 和 t 两个字符串，当它们分别被输入到空白的文本编辑器后，如果两者相等，返回 true 。# 代表退格字符。
//
// 注意：如果对空文本输入退格字符，文本继续为空。
//
//
//
// 示例 1：
//
//
//输入：s = "ab#c", t = "ad#c"
//输出：true
//解释：s 和 t 都会变成 "ac"。
//
//
// 示例 2：
//
//
//输入：s = "ab##", t = "c#d#"
//输出：true
//解释：s 和 t 都会变成 ""。
//
//
// 示例 3：
//
//
//输入：s = "a#c", t = "b"
//输出：false
//解释：s 会变成 "c"，但 t 仍然是 "b"。
//
//
//
// 提示：
//
//
// 1 <= s.length, t.length <= 200
// s 和 t 只含有小写字母以及字符 '#'
//
//
//
//
// 进阶：
//
//
// 你可以用 O(n) 的时间复杂度和 O(1) 的空间复杂度解决该问题吗？
//
//
// Related Topics 栈 双指针 字符串 模拟 👍 841 👎 0

package src

// leetcode submit region begin(Prohibit modification and deletion)
func backspaceCompare(s string, t string) bool {
	//return buildString(s, t)
	return twoPointers(s, t)
}

func buildString(s string, t string) bool {
	build := func(str string) string {
		res := make([]rune, 0)

		for _, char := range str {
			if char == '#' {
				if len(res) > 0 {
					res = res[:len(res)-1]
				}
			} else {
				res = append(res, char)
			}
		}

		return string(res)
	}

	return build(s) == build(t)
}

func twoPointers(s string, t string) bool {
	sIdx, tIdx := len(s)-1, len(t)-1
	sSharpCount, tSharpCount := 0, 0

	for sIdx >= 0 || tIdx >= 0 {
		for sIdx >= 0 {
			if s[sIdx] == '#' {
				sSharpCount++
				sIdx--
			} else if sSharpCount > 0 {
				sSharpCount--
				sIdx--
			} else {
				break
			}
		}
		for tIdx >= 0 {
			if t[tIdx] == '#' {
				tSharpCount++
				tIdx--
			} else if tSharpCount > 0 {
				tSharpCount--
				tIdx--
			} else {
				break
			}
		}

		if sIdx >= 0 && tIdx >= 0 && s[sIdx] != t[tIdx] {
			return false
		}
		if (sIdx >= 0) != (tIdx >= 0) {
			return false
		}

		sIdx--
		tIdx--
	}

	return true
}

//leetcode submit region end(Prohibit modification and deletion)
