//! Given a binary tree, find its minimum depth.
//!
//! The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
//!
//! Note: A leaf is a node with no children.
//!
//! Example:
//!
//! Given binary tree [3,9,20,null,null,15,7],
//!
//! ```ignore
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! ```
//! return its minimum depth = 2.

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

pub mod solution_bfs {
    use super::*;

    /// # 思路
    ///
    /// bfs判断遇到叶子结点时返回count
    ///
    /// 注意只有当`left, right == None, None`时可作为叶子结点，
    ///
    /// ## submission
    ///
    /// date=20200730, mem=2.5, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/373480219/
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(root);
                let mut count = 1;
                while !queue.is_empty() {
                    for _ in 0..queue.len() {
                        let root = queue.pop_front().unwrap();
                        let root = root.borrow();
                        if root.left.is_none() && root.right.is_none() {
                            return count;
                        }
                        if let Some(left) = root.left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = root.right.clone() {
                            queue.push_back(right);
                        }
                    }
                    count += 1;
                }
                count
            } else {
                0
            }
        }
    }
}

pub mod solution_dfs {
    use super::*;

    /// # 思路
    ///
    /// 在dfs递归回溯时计算每个node的深度，比较取最小值给上个节点直到root得到最小深度
    ///
    /// 注意在left, right一个节点为None时其min depth是非None depth+1，
    /// 都不是None时取min
    ///
    /// ## Submission
    ///
    /// date=20200730, mem=2.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/373460334/
    ///
    /// author=caiqi8877, references=https://leetcode.com/problems/minimum-depth-of-binary-tree/discuss/36045/My-4-Line-java-solution
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let root = root.borrow();
                let left_depth = Self::min_depth(root.left.clone());
                let right_depth = Self::min_depth(root.right.clone());
                if left_depth == 0 || right_depth == 0 {
                    left_depth + right_depth + 1
                } else {
                    left_depth.min(right_depth) + 1
                }
            } else {
                0
            }
        }
    }
}
