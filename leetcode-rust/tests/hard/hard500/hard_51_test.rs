use leetcode_exercise::leetcode::editor::cn::_51_n_queens::Solution;
use std::collections::HashSet;

#[test]
fn n_queens_1() {
    let n = 4;
    let res = Solution::solve_n_queens(n);

    assert_eq!(res.len(), 2);
    let set = HashSet::from([
        vec![
            ".Q..".to_string(),
            "...Q".to_string(),
            "Q...".to_string(),
            "..Q.".to_string(),
        ],
        vec![
            "..Q.".to_string(),
            "Q...".to_string(),
            "...Q".to_string(),
            ".Q..".to_string(),
        ],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn n_queens_2() {
    let n = 1;
    let res = Solution::solve_n_queens(n);

    assert_eq!(res.len(), 1);
    let set = HashSet::from([vec!["Q".to_string()]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn n_queens_3() {
    let n = 5;
    let res = Solution::solve_n_queens(n);

    assert_eq!(res.len(), 10);
    let set = HashSet::from([
        vec![
            "Q....".to_string(),
            "..Q..".to_string(),
            "....Q".to_string(),
            ".Q...".to_string(),
            "...Q.".to_string(), // bingo
        ],
        vec![
            "Q....".to_string(),
            "...Q.".to_string(),
            ".Q...".to_string(),
            "....Q".to_string(),
            "..Q..".to_string(),
        ],
        vec![
            ".Q...".to_string(),
            "...Q.".to_string(),
            "Q....".to_string(),
            "..Q..".to_string(),
            "....Q".to_string(), //bingo
        ],
        vec![
            ".Q...".to_string(),
            "....Q".to_string(),
            "..Q..".to_string(),
            "Q....".to_string(),
            "...Q.".to_string(), //bingo
        ],
        vec![
            "..Q..".to_string(),
            "Q....".to_string(),
            "...Q.".to_string(),
            ".Q...".to_string(),
            "....Q".to_string(),
        ],
        vec![
            "..Q..".to_string(),
            "....Q".to_string(),
            ".Q...".to_string(),
            "...Q.".to_string(),
            "Q....".to_string(),
        ],
        vec![
            "...Q.".to_string(),
            "Q....".to_string(),
            "..Q..".to_string(),
            "....Q".to_string(),
            ".Q...".to_string(), //bingo
        ],
        vec![
            "...Q.".to_string(),
            ".Q...".to_string(),
            "....Q".to_string(),
            "..Q..".to_string(),
            "Q....".to_string(), //bingo
        ],
        vec![
            "....Q".to_string(),
            ".Q...".to_string(),
            "...Q.".to_string(),
            "Q....".to_string(),
            "..Q..".to_string(),
        ],
        vec![
            "....Q".to_string(),
            "..Q..".to_string(),
            "Q....".to_string(),
            "...Q.".to_string(),
            ".Q...".to_string(), //bingo
        ],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
