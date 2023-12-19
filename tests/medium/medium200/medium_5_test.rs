use leetcode_exercise::leetcode::editor::en::_5_longest_palindromic_substring::Solution;

#[test]
fn longest_palindromic_substring() {
    let s = "babad".to_string();
    let res = Solution::longest_palindrome(s);
    assert_eq!(res, "aba"); // "bab"也可以

    let s = "cbbd".to_string();
    let res = Solution::longest_palindrome(s);
    assert_eq!(res, "bb");

    let s = "cbbijjihwzc".to_string();
    let res = Solution::longest_palindrome(s);
    assert_eq!(res, "ijji");
}
