use leetcode_rust::leetcode::editor::cn::_45_jump_game_i_i::Solution;

#[test]
fn jump_game_ii_1() {
    let nums = vec![2, 3, 1, 1, 4];
    assert_eq!(Solution::jump(nums), 2);
}

#[test]
fn jump_game_ii_2() {
    let nums = vec![2, 3, 0, 1, 4];
    assert_eq!(Solution::jump(nums), 2);
}
