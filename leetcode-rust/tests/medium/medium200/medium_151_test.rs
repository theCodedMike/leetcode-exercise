use leetcode_rust::leetcode::editor::cn::_151_reverse_words_in_a_string::Solution;

#[test]
fn reverse_words_in_a_string_1() {
    let s = "the sky is blue".to_string();
    let res = Solution::reverse_words(s);
    assert_eq!(res, "blue is sky the");
}

#[test]
fn reverse_words_in_a_string_2() {
    let s = " hello world   ".to_string();
    let res = Solution::reverse_words(s);
    assert_eq!(res, "world hello");
}

#[test]
fn reverse_words_in_a_string_3() {
    let s = "a good example".to_string();
    let res = Solution::reverse_words(s);
    assert_eq!(res, "example good a");
}
