pub mod safe {
    use std::cell::RefCell;
    use std::rc::Rc;
    use leetcode_exercise::leetcode::editor::cn::_116_populating_next_right_pointers_in_each_node::safe::Solution;
    use leetcode_exercise::binary_tree_with_next::safe::Node;
    use crate::utils::binary_tree_traversal::safe::{
        pre_order_recur as pre_order_traversal,
        in_order_recur as in_order_traversal,
        post_order_recur as post_order_traversal
    };

    fn level_vec_traversal(mut root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        while let Some(left) = root {
            let mut level_vec = vec![];

            let mut level = Some(left.clone());
            while let Some(curr) = level {
                level_vec.push(curr.borrow().val);
                level = curr.borrow_mut().next.take();
            }

            res.push(level_vec);
            root = left.borrow_mut().left.take();
        }

        res
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_1() {
        let root = None;
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res.clone()).is_empty(), true);
        assert_eq!(in_order_traversal(res.clone()).is_empty(), true);
        assert_eq!(post_order_traversal(res.clone()).is_empty(), true);
        assert_eq!(level_vec_traversal(res).is_empty(), true);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_2() {
        //   1
        let root = Node::new(1);
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res.clone()), [1]);
        assert_eq!(in_order_traversal(res.clone()), [1]);
        assert_eq!(post_order_traversal(res.clone()), [1]);
        assert_eq!(level_vec_traversal(res), [[1]]);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_3() {
        //   1
        //  / \
        // 2   3
        let root = Node::new_with_children(1, Node::new(2), Node::new(3));
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res.clone()), [1, 2, 3]);
        assert_eq!(in_order_traversal(res.clone()), [2, 1, 3]);
        assert_eq!(post_order_traversal(res.clone()), [2, 3, 1]);
        assert_eq!(level_vec_traversal(res), vec![vec![1], vec![2, 3]]);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_4() {
        //      1
        //    /   \
        //   2     3
        //  / \   / \
        // 4   5 6   7
        let root = Node::new_with_children(
            1,
            Node::new_with_children(2, Node::new(4), Node::new(5)),
            Node::new_with_children(3, Node::new(6), Node::new(7)),
        );
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res.clone()), [1, 2, 4, 5, 3, 6, 7]);
        assert_eq!(in_order_traversal(res.clone()), [4, 2, 5, 1, 6, 3, 7]);
        assert_eq!(post_order_traversal(res.clone()), [4, 5, 2, 6, 7, 3, 1]);
        assert_eq!(
            level_vec_traversal(res),
            vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]
        );
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_5() {
        //            1
        //       /        \
        //      2          3
        //    /  \       /   \
        //   4    5     6     7
        //  / \  / \   / \   / \
        // 8  9 10 11 12 13 14 15
        let root = Node::new_with_children(
            1,
            Node::new_with_children(
                2,
                Node::new_with_children(4, Node::new(8), Node::new(9)),
                Node::new_with_children(5, Node::new(10), Node::new(11)),
            ),
            Node::new_with_children(
                3,
                Node::new_with_children(6, Node::new(12), Node::new(13)),
                Node::new_with_children(7, Node::new(14), Node::new(15)),
            ),
        );
        let res = Solution::connect(root);
        assert_eq!(
            pre_order_traversal(res.clone()),
            [1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15]
        );
        assert_eq!(
            in_order_traversal(res.clone()),
            [8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert_eq!(
            post_order_traversal(res.clone()),
            [8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1]
        );
        assert_eq!(
            level_vec_traversal(res),
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11, 12, 13, 14, 15]
            ]
        );
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_6() {
        //                          1
        //               /                    \
        //             2                       3
        //       /          \             /         \
        //      4            5           6           7
        //    /   \       /    \      /    \      /    \
        //   8     9     10    11    12    13    14    15
        //  / \   / \   / \   / \   / \   / \   / \   / \
        // 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
        let root = Node::new_with_children(
            1,
            Node::new_with_children(
                2,
                Node::new_with_children(
                    4,
                    Node::new_with_children(8, Node::new(16), Node::new(17)),
                    Node::new_with_children(9, Node::new(18), Node::new(19)),
                ),
                Node::new_with_children(
                    5,
                    Node::new_with_children(10, Node::new(20), Node::new(21)),
                    Node::new_with_children(11, Node::new(22), Node::new(23)),
                ),
            ),
            Node::new_with_children(
                3,
                Node::new_with_children(
                    6,
                    Node::new_with_children(12, Node::new(24), Node::new(25)),
                    Node::new_with_children(13, Node::new(26), Node::new(27)),
                ),
                Node::new_with_children(
                    7,
                    Node::new_with_children(14, Node::new(28), Node::new(29)),
                    Node::new_with_children(15, Node::new(30), Node::new(31)),
                ),
            ),
        );
        let res = Solution::connect(root);
        let res = level_vec_traversal(res);
        assert_eq!(
            res,
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
            ]
        );
    }
}

pub mod raw_ptr {
    use std::ptr::null_mut;
    use leetcode_exercise::leetcode::editor::cn::_116_populating_next_right_pointers_in_each_node::raw_ptr::Solution;
    use leetcode_exercise::binary_tree_with_next::raw_ptr::Node;
    use crate::utils::binary_tree_traversal::raw_ptr::{
        pre_order_recur as pre_order_traversal,
        in_order_recur as in_order_traversal,
        post_order_recur as post_order_traversal
    };

    fn level_vec_util(mut root: *mut Node) -> Vec<Vec<i32>> {
        let mut res = vec![];

        while !root.is_null() {
            unsafe {
                let mut level_vec = vec![];
                let mut level = root;

                while !level.is_null() {
                    level_vec.push((*level).val);
                    level = (*level).next;
                }

                res.push(level_vec);
                root = (*root).left;
            }
        }

        res
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_1() {
        let root = null_mut();
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res).is_empty(), true);
        assert_eq!(in_order_traversal(res).is_empty(), true);
        assert_eq!(post_order_traversal(res).is_empty(), true);
        assert_eq!(level_vec_util(res).is_empty(), true);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_2() {
        //   1
        let root = Node::new(1);
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res), [1]);
        assert_eq!(in_order_traversal(res), [1]);
        assert_eq!(post_order_traversal(res), [1]);
        assert_eq!(level_vec_util(res), [[1]]);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_3() {
        //   1
        //  / \
        // 2   3
        let root = Node::new_with_children(1, Node::new(2), Node::new(3));
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res), [1, 2, 3]);
        assert_eq!(in_order_traversal(res), [2, 1, 3]);
        assert_eq!(post_order_traversal(res), [2, 3, 1]);
        assert_eq!(level_vec_util(res), vec![vec![1], vec![2, 3]]);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_4() {
        //      1
        //    /   \
        //   2     3
        //  / \   / \
        // 4   5 6   7
        let root = Node::new_with_children(
            1,
            Node::new_with_children(2, Node::new(4), Node::new(5)),
            Node::new_with_children(3, Node::new(6), Node::new(7)),
        );
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res), [1, 2, 4, 5, 3, 6, 7]);
        assert_eq!(in_order_traversal(res), [4, 2, 5, 1, 6, 3, 7]);
        assert_eq!(post_order_traversal(res), [4, 5, 2, 6, 7, 3, 1]);
        assert_eq!(
            level_vec_util(res),
            vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]
        );
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_5() {
        //            1
        //       /        \
        //      2          3
        //    /  \       /   \
        //   4    5     6     7
        //  / \  / \   / \   / \
        // 8  9 10 11 12 13 14 15
        let root = Node::new_with_children(
            1,
            Node::new_with_children(
                2,
                Node::new_with_children(4, Node::new(8), Node::new(9)),
                Node::new_with_children(5, Node::new(10), Node::new(11)),
            ),
            Node::new_with_children(
                3,
                Node::new_with_children(6, Node::new(12), Node::new(13)),
                Node::new_with_children(7, Node::new(14), Node::new(15)),
            ),
        );
        let res = Solution::connect(root);
        assert_eq!(
            pre_order_traversal(res),
            [1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15]
        );
        assert_eq!(
            in_order_traversal(res),
            [8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert_eq!(
            post_order_traversal(res),
            [8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1]
        );
        assert_eq!(
            level_vec_util(res),
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11, 12, 13, 14, 15]
            ]
        );
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_6() {
        //                          1
        //               /                    \
        //             2                       3
        //       /          \             /         \
        //      4            5           6           7
        //    /   \       /    \      /    \      /    \
        //   8     9     10    11    12    13    14    15
        //  / \   / \   / \   / \   / \   / \   / \   / \
        // 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
        let root = Node::new_with_children(
            1,
            Node::new_with_children(
                2,
                Node::new_with_children(
                    4,
                    Node::new_with_children(8, Node::new(16), Node::new(17)),
                    Node::new_with_children(9, Node::new(18), Node::new(19)),
                ),
                Node::new_with_children(
                    5,
                    Node::new_with_children(10, Node::new(20), Node::new(21)),
                    Node::new_with_children(11, Node::new(22), Node::new(23)),
                ),
            ),
            Node::new_with_children(
                3,
                Node::new_with_children(
                    6,
                    Node::new_with_children(12, Node::new(24), Node::new(25)),
                    Node::new_with_children(13, Node::new(26), Node::new(27)),
                ),
                Node::new_with_children(
                    7,
                    Node::new_with_children(14, Node::new(28), Node::new(29)),
                    Node::new_with_children(15, Node::new(30), Node::new(31)),
                ),
            ),
        );
        let res = Solution::connect(root);
        let res = level_vec_util(res);
        assert_eq!(
            res,
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
            ]
        );
    }
}

pub mod nonnull {
    use std::ptr::NonNull;
    use leetcode_exercise::leetcode::editor::cn::_116_populating_next_right_pointers_in_each_node::nonnull::Solution;
    use leetcode_exercise::binary_tree_with_next::nonnull::Node;
    use crate::utils::binary_tree_traversal::nonnull::{
        pre_order_recur as pre_order_traversal,
        in_order_recur as in_order_traversal,
        post_order_recur as post_order_traversal
    };

    fn level_vec_util(mut root: Option<NonNull<Node>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        while let Some(left) = root {
            unsafe {
                let mut level_vec = vec![];
                let mut level = Some(left);

                while let Some(curr) = level {
                    level_vec.push(curr.as_ref().val);
                    level = curr.as_ref().next;
                }

                res.push(level_vec);
                root = left.as_ref().left;
            }
        }

        res
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_1() {
        let root = None;
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res).is_empty(), true);
        assert_eq!(in_order_traversal(res).is_empty(), true);
        assert_eq!(post_order_traversal(res).is_empty(), true);
        assert_eq!(level_vec_util(res).is_empty(), true);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_2() {
        //   1
        let root = Node::new(1);
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res), [1]);
        assert_eq!(in_order_traversal(res), [1]);
        assert_eq!(post_order_traversal(res), [1]);
        assert_eq!(level_vec_util(res), [[1]]);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_3() {
        //   1
        //  / \
        // 2   3
        let root = Node::new_with_children(1, Node::new(2), Node::new(3));
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res), [1, 2, 3]);
        assert_eq!(in_order_traversal(res), [2, 1, 3]);
        assert_eq!(post_order_traversal(res), [2, 3, 1]);
        assert_eq!(level_vec_util(res), vec![vec![1], vec![2, 3]]);
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_4() {
        //      1
        //    /   \
        //   2     3
        //  / \   / \
        // 4   5 6   7
        let root = Node::new_with_children(
            1,
            Node::new_with_children(2, Node::new(4), Node::new(5)),
            Node::new_with_children(3, Node::new(6), Node::new(7)),
        );
        let res = Solution::connect(root);
        assert_eq!(pre_order_traversal(res), [1, 2, 4, 5, 3, 6, 7]);
        assert_eq!(in_order_traversal(res), [4, 2, 5, 1, 6, 3, 7]);
        assert_eq!(post_order_traversal(res), [4, 5, 2, 6, 7, 3, 1]);
        assert_eq!(
            level_vec_util(res),
            vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]
        );
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_5() {
        //            1
        //       /        \
        //      2          3
        //    /  \       /   \
        //   4    5     6     7
        //  / \  / \   / \   / \
        // 8  9 10 11 12 13 14 15
        let root = Node::new_with_children(
            1,
            Node::new_with_children(
                2,
                Node::new_with_children(4, Node::new(8), Node::new(9)),
                Node::new_with_children(5, Node::new(10), Node::new(11)),
            ),
            Node::new_with_children(
                3,
                Node::new_with_children(6, Node::new(12), Node::new(13)),
                Node::new_with_children(7, Node::new(14), Node::new(15)),
            ),
        );
        let res = Solution::connect(root);
        assert_eq!(
            pre_order_traversal(res),
            [1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15]
        );
        assert_eq!(
            in_order_traversal(res),
            [8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert_eq!(
            post_order_traversal(res),
            [8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1]
        );
        assert_eq!(
            level_vec_util(res),
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11, 12, 13, 14, 15]
            ]
        );
    }

    #[test]
    fn populating_next_right_pointers_in_each_node_6() {
        //                          1
        //               /                    \
        //             2                       3
        //       /          \             /         \
        //      4            5           6           7
        //    /   \       /    \      /    \      /    \
        //   8     9     10    11    12    13    14    15
        //  / \   / \   / \   / \   / \   / \   / \   / \
        // 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
        let root = Node::new_with_children(
            1,
            Node::new_with_children(
                2,
                Node::new_with_children(
                    4,
                    Node::new_with_children(8, Node::new(16), Node::new(17)),
                    Node::new_with_children(9, Node::new(18), Node::new(19)),
                ),
                Node::new_with_children(
                    5,
                    Node::new_with_children(10, Node::new(20), Node::new(21)),
                    Node::new_with_children(11, Node::new(22), Node::new(23)),
                ),
            ),
            Node::new_with_children(
                3,
                Node::new_with_children(
                    6,
                    Node::new_with_children(12, Node::new(24), Node::new(25)),
                    Node::new_with_children(13, Node::new(26), Node::new(27)),
                ),
                Node::new_with_children(
                    7,
                    Node::new_with_children(14, Node::new(28), Node::new(29)),
                    Node::new_with_children(15, Node::new(30), Node::new(31)),
                ),
            ),
        );
        let res = Solution::connect(root);
        let res = level_vec_util(res);
        assert_eq!(
            res,
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
            ]
        );
    }
}
