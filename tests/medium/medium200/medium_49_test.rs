use leetcode_exercise::leetcode::editor::cn::_49_group_anagrams::Solution;

#[test]
fn group_anagrams_test() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let res = Solution::group_anagrams(strs);
    println!("{:?}", res);

    let strs = vec!["".to_string()];
    let res = Solution::group_anagrams(strs);
    assert_eq!(res, vec![vec!["".to_string()]]);

    let strs = vec!["a".to_string()];
    let res = Solution::group_anagrams(strs);
    assert_eq!(res, vec![vec!["a".to_string()]]);
}
