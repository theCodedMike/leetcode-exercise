use leetcode_exercise::leetcode::editor::cn::_429_n_ary_tree_level_order_traversal::Solution;
use leetcode_exercise::n_ary_tree::Node;

#[test]
fn n_ary_tree_level_order_traversal_1() {
    //        1
    //     /  |  \
    //    3   2   4
    //   / \
    //  5   6
    let root = Node::new_with_children(
        1,
        vec![
            Node::new_with_children(3, vec![Node::new(5), Node::new(6)]),
            Node::new(2),
            Node::new(4),
        ],
    );
    let res = Solution::level_order(root);
    assert_eq!(res, vec![vec![1], vec![3, 2, 4], vec![5, 6]]);
}

#[test]
fn n_ary_tree_level_order_traversal_2() {
    //        1
    //   /   / \   \
    //  2   3   4    5
    //     / \  |   / \
    //    6  7  8  9   10
    //       |  |  |
    //      11 12 13
    //      |
    //      14
    let root = Node::new_with_children(
        1,
        vec![
            Node::new(2),
            Node::new_with_children(
                3,
                vec![
                    Node::new(6),
                    Node::new_with_children(
                        7,
                        vec![Node::new_with_children(11, vec![Node::new(14)])],
                    ),
                ],
            ),
            Node::new_with_children(4, vec![Node::new_with_children(8, vec![Node::new(12)])]),
            Node::new_with_children(
                5,
                vec![
                    Node::new_with_children(9, vec![Node::new(13)]),
                    Node::new(10),
                ],
            ),
        ],
    );
    let res = Solution::level_order(root);
    assert_eq!(
        res,
        vec![
            vec![1],
            vec![2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13],
            vec![14]
        ]
    );
}
