use leetcode_exercise::leetcode::editor::cn::_149_max_points_on_a_line::Solution;

#[test]
fn max_points_on_a_line() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    let res = Solution::max_points(points);
    assert_eq!(res, 3);

    let points = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    let res = Solution::max_points(points);
    assert_eq!(res, 4);

    let points = vec![vec![3, 3], vec![1, 4], vec![1, 1], vec![2, 1], vec![2, 2]];
    let res = Solution::max_points(points);
    assert_eq!(res, 3);

    let points = vec![
        vec![0, 0],
        vec![4, 5],
        vec![7, 8],
        vec![8, 9],
        vec![5, 6],
        vec![3, 4],
        vec![1, 1],
    ];
    let res = Solution::max_points(points);
    assert_eq!(res, 5);
}

#[test]
fn max_points_on_a_line2() {
    let points = vec![
        vec![7, 3],
        vec![19, 19],
        vec![-16, 3],
        vec![13, 17],
        vec![-18, 1],
        vec![-18, -17],
        vec![13, -3],
        vec![3, 7],
        vec![-11, 12],
        vec![7, 19],
        vec![19, -12],
        vec![20, -18],
        vec![-16, -15],
        vec![-10, -15],
        vec![-16, -18],
        vec![-14, -1],
        vec![18, 10],
        vec![-13, 8],
        vec![7, -5],
        vec![-4, -9],
        vec![-11, 2],
        vec![-9, -9],
        vec![-5, -16],
        vec![10, 14],
        vec![-3, 4],
        vec![1, -20],
        vec![2, 16],
        vec![0, 14],
        vec![-14, 5],
        vec![15, -11],
        vec![3, 11],
        vec![11, -10],
        vec![-1, -7],
        vec![16, 7],
        vec![1, -11],
        vec![-8, -3],
        vec![1, -6],
        vec![19, 7],
        vec![3, 6],
        vec![-1, -2],
        vec![7, -3],
        vec![-6, -8],
        vec![7, 1],
        vec![-15, 12],
        vec![-17, 9],
        vec![19, -9],
        vec![1, 0],
        vec![9, -10],
        vec![6, 20],
        vec![-12, -4],
        vec![-16, -17],
        vec![14, 3],
        vec![0, -1],
        vec![-18, 9],
        vec![-15, 15],
        vec![-3, -15],
        vec![-5, 20],
        vec![15, -14],
        vec![9, -17],
        vec![10, -14],
        vec![-7, -11],
        vec![14, 9],
        vec![1, -1],
        vec![15, 12],
        vec![-5, -1],
        vec![-17, -5],
        vec![15, -2],
        vec![-12, 11],
        vec![19, -18],
        vec![8, 7],
        vec![-5, -3],
        vec![-17, -1],
        vec![-18, 13],
        vec![15, -3],
        vec![4, 18],
        vec![-14, -15],
        vec![15, 8],
        vec![-18, -12],
        vec![-15, 19],
        vec![-9, 16],
        vec![-9, 14],
        vec![-12, -14],
        vec![-2, -20],
        vec![-3, -13],
        vec![10, -7],
        vec![-2, -10],
        vec![9, 10],
        vec![-1, 7],
        vec![-17, -6],
        vec![-15, 20],
        vec![5, -17],
        vec![6, -6],
        vec![-11, -8],
    ];
    let res = Solution::max_points(points);
    assert_eq!(res, 6);
}
