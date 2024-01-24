//Given the roots of two binary trees root and subRoot, return true if there is
//a subtree of root with the same structure and node values of subRoot and false
//otherwise.
//
// A subtree of a binary tree tree is a tree that consists of a node in tree
//and all of this node's descendants. The tree tree could also be considered as a
//subtree of itself.
//
//
// Example 1:
//
//
//Input: root = [3,4,5,1,2], subRoot = [4,1,2]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in the root tree is in the range [1, 2000].
// The number of nodes in the subRoot tree is in the range [1, 1000].
// -10⁴ <= root.val <= 10⁴
// -10⁴ <= subRoot.val <= 10⁴
//
//
// Related Topics 树 深度优先搜索 二叉树 字符串匹配 哈希函数 👍 1000 👎 0

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        //Self::dfs_recur_match(root, sub_root)

        Self::dfs_sequence_match(root, sub_root)

        //Self::tree_hash(root, sub_root)
    }

    ///
    /// Time Complexity: O(|r| * |s|)
    /// Space Complexity: O(max(dr, ds))
    ///
    fn dfs_recur_match(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        const CHECK: fn(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) -> bool =
            |root, sub_root| match (root, sub_root) {
                (None, None) => true,
                (Some(root), Some(sub_root)) => {
                    if root.borrow().val != sub_root.borrow().val {
                        return false;
                    }

                    CHECK(root.borrow().left.clone(), sub_root.borrow().left.clone())
                        && CHECK(root.borrow().right.clone(), sub_root.borrow().right.clone())
                }
                _ => false,
            };
        const DFS: fn(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) -> bool =
            |root, sub_root| match root {
                None => false,
                Some(root) => {
                    let left = root.borrow().left.clone();
                    let right = root.borrow().right.clone();

                    CHECK(Some(root), sub_root.clone())
                        || DFS(left, sub_root.clone())
                        || DFS(right, sub_root)
                }
            };

        DFS(root, sub_root)
    }

    ///
    /// Time Complexity: O(|r| + |s|)
    /// Space Complexity: O(|r| + |s|)
    ///
    fn dfs_sequence_match(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        const GET_MAX_ELEMENT: fn(&Option<Rc<RefCell<TreeNode>>>) -> i32 = |root| match root {
            None => i32::MIN,
            Some(curr) => {
                let max_child = std::cmp::max(
                    GET_MAX_ELEMENT(&curr.borrow().left),
                    GET_MAX_ELEMENT(&curr.borrow().right),
                );

                std::cmp::max(max_child, curr.borrow().val)
            }
        };

        let max_elem = std::cmp::max(GET_MAX_ELEMENT(&root), GET_MAX_ELEMENT(&sub_root));
        let (l_null, r_null) = (max_elem + 1, max_elem + 2);

        let get_dfs_order = |root| {
            let mut vals = vec![];
            const GET_DFS_ORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut Vec<i32>, i32, i32) =
                |root, vals, l_null, r_null| {
                    if let Some(curr) = root {
                        vals.push(curr.borrow().val);
                        let left = curr.borrow_mut().left.take();
                        let right = curr.borrow_mut().right.take();

                        if left.is_some() {
                            GET_DFS_ORDER(left, vals, l_null, r_null);
                        } else {
                            vals.push(l_null);
                        }
                        if right.is_some() {
                            GET_DFS_ORDER(right, vals, l_null, r_null);
                        } else {
                            vals.push(r_null);
                        }
                    }
                };

            GET_DFS_ORDER(root, &mut vals, l_null, r_null);

            vals
        };

        let root_vals = get_dfs_order(root);
        let sub_vals = get_dfs_order(sub_root);

        let kmp = || {
            let root_len = root_vals.len();
            let sub_len = sub_vals.len();
            let mut fail = vec![-1; sub_len];

            let mut j = -1_i32;
            for i in 1..sub_len {
                while j != -1 && sub_vals[i] != sub_vals[(j + 1) as usize] {
                    j = fail[j as usize]
                }
                if sub_vals[i] == sub_vals[(j + 1) as usize] {
                    j += 1;
                }
                fail[i] = j;
            }

            j = -1;
            for i in 0..root_len {
                while j != -1 && root_vals[i] != sub_vals[(j + 1) as usize] {
                    j = fail[j as usize];
                }
                if root_vals[i] == sub_vals[(j + 1) as usize] {
                    j += 1;
                }
                if (j + 1) as usize == sub_len {
                    return true;
                }
            }

            false
        };

        kmp()
    }

    ///
    /// Time Complexity: O(|r| + |s|)
    /// Space Complexity: O(|r| + |s|)
    ///
    fn tree_hash(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        const MAX_N: usize = 1000 + 5;
        const MOD: i32 = 1e9 as i32 + 7;

        let get_primes = || {
            let mut vis = vec![false; MAX_N];
            let mut primes = vec![0; MAX_N];
            vis[0] = true;
            vis[1] = true;
            let mut tot = 0;

            for i in 2..MAX_N {
                if !vis[i] {
                    tot += 1;
                    primes[tot] = i;
                }
                let mut j = 1;
                while j <= tot && i * primes[j] < MAX_N {
                    vis[i * primes[j]] = true;
                    if i % primes[j] == 0 {
                        break;
                    }
                    j += 1;
                }
            }

            primes
        };
        let primes = get_primes();

        let get_hashmap = |root, key| {
            let mut map = HashMap::new();

            const DFS: fn(
                Option<Rc<RefCell<TreeNode>>>,
                &mut HashMap<(usize, i32, bool, Option<i32>), (i32, usize)>,
                &Vec<usize>,
                (usize, i32, bool, Option<i32>),
            ) = |root, map, primes, key| {
                if let Some(curr) = root {
                    let root_val = curr.borrow().val;
                    map.insert(key, (root_val, 1));

                    if let Some(left) = curr.borrow().left.clone() {
                        let l_key = (key.0 + 1, left.borrow().val, true, Some(root_val));
                        DFS(Some(left), map, primes, l_key);

                        let l_sub_tree = map[&l_key].1;
                        let tmp_val =
                            (31_i128 * map[&l_key].0 as i128 * primes[map[&l_key].1] as i128)
                                % MOD as i128;
                        let l_hash = (map[&key].0 + tmp_val as i32) % MOD;
                        map.get_mut(&key).map(|(hash, sub_tree)| {
                            *sub_tree += l_sub_tree;
                            *hash = l_hash;
                        });
                    }

                    if let Some(right) = curr.borrow().right.clone() {
                        let r_key = (key.0 + 1, right.borrow().val, false, Some(root_val));
                        DFS(Some(right), map, primes, r_key);
                        let r_sub_tree = map[&r_key].1;
                        let tmp_val =
                            (179_i128 * map[&r_key].0 as i128 * primes[map[&r_key].1] as i128)
                                % MOD as i128;
                        let r_hash = (map[&key].0 + tmp_val as i32) % MOD;
                        map.get_mut(&key).map(|(hash, sub_tree)| {
                            *sub_tree += r_sub_tree;
                            *hash = r_hash;
                        });
                    }
                }
            };

            DFS(root, &mut map, &primes, key);

            map
        };
        let root_key = (1, root.as_ref().unwrap().borrow().val, false, None);
        let root_map = get_hashmap(root, root_key);
        let sub_key = (1, sub_root.as_ref().unwrap().borrow().val, false, None);
        let sub_map = get_hashmap(sub_root, sub_key);

        let sub_hash = sub_map[&sub_key].0;
        root_map.values().any(|v| v.0 == sub_hash)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
