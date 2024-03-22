use leetcode_rust::leetcode::editor::cn::_763_partition_labels::Solution;

#[test]
fn partition_labels_1() {
    let s = "ababcbacadefegdehijhklij".to_string();
    let res = Solution::partition_labels(s);
    // "ababcbaca" + "defegde" + "hijhklij"
    assert_eq!(res, [9, 7, 8]);
}

#[test]
fn partition_labels_2() {
    let s = "eccbbbbdec".to_string();
    let res = Solution::partition_labels(s);
    // "eccbbbbdec"
    assert_eq!(res, [10]);
}

#[test]
fn partition_labels_3() {
    let s = "caedbdedda".to_string();
    let res = Solution::partition_labels(s);
    // "c" + "aedbdedda"
    assert_eq!(res, [1, 9]);
}

#[test]
fn partition_labels_4() {
    let s = "eaaaabaaec".to_string();
    let res = Solution::partition_labels(s);
    // "eaaaabaae" + "c"
    assert_eq!(res, [9, 1]);
}
