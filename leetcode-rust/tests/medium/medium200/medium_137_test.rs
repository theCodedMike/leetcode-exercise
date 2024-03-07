use leetcode_rust::leetcode::editor::cn::_137_single_number_i_i::Solution;

#[test]
fn single_number_ii() {
    let nums = vec![2, 2, 3, 2];
    let res = Solution::use_bit_manipulation2(nums);
    assert_eq!(res, 3);

    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    let res = Solution::use_bit_manipulation2(nums);
    assert_eq!(res, 99);
}
