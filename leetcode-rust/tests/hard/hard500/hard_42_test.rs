use leetcode_rust::leetcode::editor::cn::_42_trapping_rain_water::Solution;

#[test]
fn trapping_rain_water() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let res = Solution::trap(height);
    assert_eq!(res, 6);

    let height = vec![4, 2, 0, 3, 2, 5];
    let res = Solution::trap(height);
    assert_eq!(res, 9);

    let height = vec![4, 2, 3];
    let res = Solution::trap(height);
    assert_eq!(res, 1);
}
