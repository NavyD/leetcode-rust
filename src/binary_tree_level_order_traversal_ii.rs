//! Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
//!
//! For example:
//! Given binary tree [3,9,20,null,null,15,7],
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! return its bottom-up level order traversal as:
//! [
//!   [15,7],
//!   [9,20],
//!   [3]
//! ]

// #[derive(Debug, PartialEq, Eq)]
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
    /// dfs回溯时插入
    /// 
    /// 如何确定下标？
    /// 
    /// 对于同一层，left和right，不能直接回溯下标0开始，由于先在left回溯，right可能存在更多的子结点，
    /// right可有更多层
    /// 
    /// ```ignore
    /// fn level_order(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) -> usize {
    /// ```
    /// 
    /// ## Submissions
    /// 
    /// date=20200714, mem=2.3, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/366304542/
    /// 
    /// author=SOY, references=https://leetcode.com/problems/binary-tree-level-order-traversal-ii/discuss/34981/My-DFS-and-BFS-java-solution
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N^2)
    pub struct Solution;

    impl Solution {
        pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut res = vec![];
            Self::level_order(&root, &mut res, 0);
            res
        }

        fn level_order(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>, level: usize) {
            if let Some(root) = root {
                if level >= res.len() {
                    res.insert(0, vec![]);
                }
                Self::level_order(&root.borrow().left, res, level + 1);
                Self::level_order(&root.borrow().right, res, level + 1);
                let i = res.len() - level - 1;
                res.get_mut(i).unwrap().push(root.borrow().val);
            }
        }
    }
}
