use leetcode_rust::leetcode::editor::cn::_406_queue_reconstruction_by_height::Solution;

#[test]
fn queue_reconstruction_by_height_1() {
    let people = vec![
        vec![7, 0],
        vec![4, 4],
        vec![7, 1],
        vec![5, 0],
        vec![6, 1],
        vec![5, 2],
    ];
    let res = Solution::reconstruct_queue(people);
    let exp = [[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]];
    assert_eq!(res, exp);
}

#[test]
fn queue_reconstruction_by_height_2() {
    let people = vec![
        vec![6, 0],
        vec![5, 0],
        vec![4, 0],
        vec![3, 2],
        vec![2, 2],
        vec![1, 4],
    ];
    let res = Solution::reconstruct_queue(people);
    let exp = [[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]];
    assert_eq!(res, exp);
}
