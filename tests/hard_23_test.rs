use leetcode_exercise::leetcode::editor::en::_23_merge_k_sorted_lists::{ListNode, Solution};

/// cargo test -- --show-output merge_k_sorted_lists_test
#[test]
fn merge_k_sorted_lists_test() {
    let mut lists = vec![];
    let node_5 = ListNode::new(5);
    let node_4_5 = ListNode::build(4, Some(Box::new(node_5)));
    let node_1_4_5 = ListNode::build(1, Some(Box::new(node_4_5)));
    lists.push(Some(Box::new(node_1_4_5)));

    let node_4 = ListNode::new(4);
    let node_3_4 = ListNode::build(3, Some(Box::new(node_4)));
    let node_1_3_4 = ListNode::build(1, Some(Box::new(node_3_4)));
    lists.push(Some(Box::new(node_1_3_4)));

    let node_6 = ListNode::new(6);
    let node_2_6 = ListNode::build(2, Some(Box::new(node_6)));
    lists.push(Some(Box::new(node_2_6)));

    let res = Solution::merge_k_lists(lists);
    println!("{:?}", res);
    // Some(ListNode { val: 1, next:
    //      Some(ListNode { val: 1, next:
    //           Some(ListNode { val: 2, next:
    //                Some(ListNode { val: 3, next:
    //                     Some(ListNode { val: 4, next:
    //                          Some(ListNode { val: 4, next:
    //                               Some(ListNode { val: 5, next:
    //                                    Some(ListNode { val: 6, next: None })
    //                                             })
    //                                        })
    //                                   })
    //                              })
    //                         })
    //                    })
    //                })
}
