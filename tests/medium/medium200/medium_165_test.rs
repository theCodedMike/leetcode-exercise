use leetcode_exercise::leetcode::editor::en::_165_compare_version_numbers::Solution;

#[test]
fn compare_version_numbers() {
    let version1 = "1.0".to_string();
    let version2 = "1.1".to_string();
    let res = Solution::compare_version(version1, version2);
    assert_eq!(res, -1);

    let version1 = "1.2".to_string();
    let version2 = "1.10".to_string();
    let res = Solution::compare_version(version1, version2);
    assert_eq!(res, -1);

    let version1 = "1".to_string();
    let version2 = "1.1".to_string();
    let res = Solution::compare_version(version1, version2);
    assert_eq!(res, -1);
}
