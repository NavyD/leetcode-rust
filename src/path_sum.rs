//! Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
//!
//! Note: A leaf is a node with no children.
//!
//! Example:
//!
//! Given the below binary tree and sum = 22,
//!
//! ```ignore
//!       5
//!      / \
//!     4   8
//!    /   / \
//!   11  13  4
//!  /  \      \
//! 7    2      1
//! ```
//! return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.

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

pub mod solution_dfs {
    use super::*;

    /// # 思路
    ///
    /// 当root是叶子节点时且路径和=sum
    ///
    /// dfs如何计算path sum
    ///
    /// 用先序遍历递归计算root.val-sum作为new sum
    /// 
    /// ## Submission
    /// 
    /// date=20200805, mem=2.7, mem_beats=2.6, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/376461021/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
            if let Some(root) = root {
                let root = root.borrow();
                let sum = sum - root.val;
                if root.left.is_none() && root.right.is_none() {
                    sum == 0
                } else {
                    Self::has_path_sum(root.left.clone(), sum)
                        || Self::has_path_sum(root.right.clone(), sum)
                }
            } else {
                false
            }
        }
    }
}

pub mod solution_bfs {
    use super::*;
    /// bfs如何记录路径和与节点对应
    pub struct Solution;

    impl Solution {
        pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
            false
        }
    }
}