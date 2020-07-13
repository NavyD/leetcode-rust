//! Given a binary tree, find its maximum depth.
//! 
//! The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//! 
//! Note: A leaf is a node with no children.
//! 
//! Example:
//! 
//! Given binary tree [3,9,20,null,null,15,7],
//! 
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! return its depth = 3.

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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

pub mod solution_recursion {
    use super::*;
    /// # 思路
    /// 
    /// 递归计算出深度
    /// 
    /// ## Submissions
    /// 
    /// date=20200713, mem=2.7, mem_beats=80.95, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/365857015/
    /// 
    /// ## 复杂度
    /// 
    pub struct Solution;
    
    impl Solution {
        pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let None = root {
                return 0;
            } else {
                let root = root.to_owned().unwrap();
                let left_count = Self::max_depth(root.borrow().left.clone()) + 1;
                let right_count = Self::max_depth(root.borrow().right.clone()) + 1;
                return left_count.max(right_count)
            }
        }
    }
}