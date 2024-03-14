use leetcode_rust::leetcode::editor::cn::_55_jump_game::Solution;

#[test]
fn jump_game_1() {
    let nums = vec![2, 3, 1, 1, 4];
    let res = Solution::can_jump(nums);
    assert_eq!(res, true);
}

#[test]
fn jump_game_2() {
    let nums = vec![3, 2, 1, 0, 4];
    let res = Solution::can_jump(nums);
    assert_eq!(res, false);
}

#[test]
fn jump_game_3() {
    let nums = vec![0];
    let res = Solution::can_jump(nums);
    assert_eq!(res, true);
}

#[test]
fn jump_game_4() {
    let nums = vec![0, 1];
    let res = Solution::can_jump(nums);
    assert_eq!(res, false);
}

#[test]
fn jump_game_5() {
    let nums = vec![2, 0, 0];
    let res = Solution::can_jump(nums);
    assert_eq!(res, true);
}
