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

        fn level_order(
            root: &Option<Rc<RefCell<TreeNode>>>,
            res: &mut Vec<Vec<i32>>,
            level: usize,
        ) {
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

pub mod solution_bfs {
    use super::*;

    /// # 思路
    ///
    /// bfs一层层遍历所有结点
    ///
    /// ## Submissions
    ///
    /// date=20200718, mem=2.2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/368224847/
    ///
    /// author=navyd
    ///
    /// ## 复杂度
    ///
    /// 树的高度为M，个数为N
    ///
    /// - 时间：O(N)
    /// - 空间：O(N*M)
    pub struct Solution;

    impl Solution {
        pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut res = vec![];
            if root.is_none() {
                return res;
            }
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root.unwrap());
            while queue.front().is_some() {
                let mut vals = vec![];
                // 当前层
                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    vals.push(node.val);
                    // 子结点
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
                // 插入当前到最前面
                res.insert(0, vals);
            }
            res
        }
    }
}
