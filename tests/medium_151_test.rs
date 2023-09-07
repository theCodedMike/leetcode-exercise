use leetcode_exercise::leetcode::editor::en::_151_reverse_words_in_a_string::Solution;

#[test]
fn reverse_words_in_a_string() {
    let s = "the sky is blue".to_string();
    let res = Solution::reverse_words(s);
    assert_eq!(res, "blue is sky the");

    let s = " hello world ".to_string();
    let res = Solution::reverse_words(s);
    assert_eq!(res, "world hello");

    let s = "a good example".to_string();
    let res = Solution::reverse_words(s);
    assert_eq!(res, "example good a");
}
