//! Given a binary tree, return all root-to-leaf paths.
//!
//! Note: A leaf is a node with no children.
//!
//! Example:
//!
//! Input:
//! ```ignore
//!    1
//!  /   \
//! 2     3
//!  \
//!   5
//! ```
//! Output: ["1->2->5", "1->3"]
//!
//! Explanation: All root-to-leaf paths are: 1->2->5, 1->3

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

/// 总结
/// 
/// dfs时想到root可由一个string分离，但没有更多的对应
/// 每条path与结点了，反而是用返回值->Option<String>表示
/// 对应关系，导致纠结于后序string::insert(0, str)与处理
/// 删除最后'->'
pub mod solution_dfs {
    use super::*;
    /// # 思路
    /// 
    /// 在递归遍历二叉树时，需要考虑当前的节点和它的孩子节点。
    /// 如果当前的节点不是叶子节点，则在当前的路径末尾添加该节点，并递归遍历该节点的每一个孩子节点。
    /// 如果当前的节点是叶子节点，则在当前的路径末尾添加该节点后，就得到了一条从根节点到叶子节点的路径，
    /// 可以把该路径加入到答案中。
    ///
    /// ## Submission
    /// 
    /// date=20200807, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/95524442/
    /// 
    /// author=力扣 (LeetCode), references=https://leetcode-cn.com/problems/binary-tree-paths/solution/er-cha-shu-de-suo-you-lu-jing-by-leetcode/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
            let mut paths = vec![];
            Self::dfs_paths(root, &mut paths, String::new());
            paths
        }

        fn dfs_paths(root: Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<String>, mut root_path: String) {
            if let Some(root) = root {
                let root = root.borrow();
                root_path.push_str(&root.val.to_string());
                if root.left.is_none() && root.right.is_none() {
                    paths.push(root_path);
                } else {
                    root_path.push_str("->");
                    Self::dfs_paths(root.left.clone(), paths, root_path.clone());
                    Self::dfs_paths(root.right.clone(), paths, root_path);
                }
            }
        }
    }
}

/// 总结
/// 
/// 没有考虑如何求出path，当是叶子结点时加入结果集，没有对结果考虑，导致只有固定的
/// stack bfs结构。
pub mod solution_bfs {
    use super::*;

    /// # 思路
    /// 
    /// 在bfs中用path stack对应node stack
    /// 
    /// ## Submission
    /// 
    /// date=20200807, mem=2.2, mem_beats=33.33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/95531421/
    /// 
    /// author=力扣 (LeetCode), references=https://leetcode-cn.com/problems/binary-tree-paths/solution/er-cha-shu-de-suo-you-lu-jing-by-leetcode/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
            let mut paths = vec![];
            if let Some(root) = root {
                let mut path_stack = std::collections::VecDeque::new();
                path_stack.push_back(root.borrow().val.to_string());
                let mut node_stack = std::collections::VecDeque::new();
                node_stack.push_back(root);
                while !node_stack.is_empty() {
                    for _ in 0..node_stack.len() {
                        let root = node_stack.pop_front().unwrap();
                        let root = root.borrow();
                        let mut path = path_stack.pop_front().unwrap();
                        if root.left.is_none() && root.right.is_none() {
                            paths.push(path);
                            break;
                        } 
                        path.push_str("->");
                        if let Some(left) = root.left.clone() {
                            path_stack.push_back(path.clone() + &left.borrow().val.to_string());
                            node_stack.push_back(left);
                        }
                        if let Some(right) = root.right.clone() {
                            path_stack.push_back(path + &right.borrow().val.to_string());
                            node_stack.push_back(right);
                        }
                        
                    }
                }
            }
            paths
        }
    }
}
