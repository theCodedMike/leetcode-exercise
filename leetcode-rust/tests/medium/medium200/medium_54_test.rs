use leetcode_rust::leetcode::editor::cn::_54_spiral_matrix::Solution;

#[test]
fn spiral_matrix() {
    let matrix = vec![vec![1]];
    let res = Solution::spiral_order(matrix);
    assert_eq!(res, [1]);

    let matrix = vec![vec![1, 2], vec![3, 4]];
    let res = Solution::spiral_order(matrix);
    assert_eq!(res, [1, 2, 4, 3]);

    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let res = Solution::spiral_order(matrix);
    assert_eq!(res, [1, 2, 3, 6, 9, 8, 7, 4, 5]);

    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let res = Solution::spiral_order(matrix);
    assert_eq!(res, [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);

    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];
    let res = Solution::spiral_order(matrix);
    assert_eq!(res, [1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8]);

    let matrix = vec![vec![7], vec![9], vec![6]];
    let res = Solution::spiral_order(matrix);
    assert_eq!(res, [7, 9, 6]);

    let matrix = vec![vec![7, 9, 6]];
    let res = Solution::spiral_order(matrix);
    assert_eq!(res, [7, 9, 6]);
}
