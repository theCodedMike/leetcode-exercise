use leetcode_rust::leetcode::editor::cn::_60_permutation_sequence::Solution;

#[test]
fn permutation_sequence() {
    let n = 3;
    let k = 3;
    let res = Solution::get_permutation(n, k);
    assert_eq!(res, "213");

    let n = 4;
    let k = 9;
    let res = Solution::get_permutation(n, k);
    assert_eq!(res, "2314");
}
