use leetcode_exercise::leetcode::editor::cn::_30_substring_with_concatenation_of_all_words::Solution;

#[test]
fn substring_with_concatenation_of_all_words() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    let res = Solution::find_substring(s, words);
    assert_eq!(res, [0, 9]);

    let s = "wordgoodgoodgoodbestword".to_string();
    let words = vec![
        "word".to_string(),
        "good".to_string(),
        "best".to_string(),
        "word".to_string(),
    ];
    let res = Solution::find_substring(s, words);
    assert_eq!(res, []);

    let s = "barfoofoobarthefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string(), "the".to_string()];
    let res = Solution::find_substring(s, words);
    assert_eq!(res, [6, 9, 12]);
}
