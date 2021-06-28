//! Find the sum of all left leaves in a given binary tree.
//!
//! Example:
//! ```ignore
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! ```
//! There are two left leaves in the binary tree, with values 9 and 15 respectively. Return 24.

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
/// 用`dfs_sum(root: Option<Rc<RefCell<TreeNode>>>, parent: Option<Rc<RefCell<TreeNode>>>) -> i32 {`
/// 中`parent.borrow().left == root.clone()`判断root是否是左子叶，在最后一个测试用例中失败了
/// `[-6,8,-4,8,-5,-1,null,-9,9,8,8,null,null,-5,6,null,null,null,-4,null,4,null,null,8,8,null,null,null,5,null,null,null,null,null,-9]`，
/// 很明显是rust == derive eq属性方面出现的，不清楚
pub mod solution_dfs {
    use super::*;
    /// # 思路
    ///
    /// dfs找左子叶val递归相加得到和
    ///
    /// 如何判断root是左子叶？
    ///
    /// 递归时传递参数is_left，当root.left递归时is_left=true，反之为false
    ///
    /// ## Submission
    ///
    /// date=20200808, mem=2.2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/95889124/
    ///
    /// author=lastwhisper, references=https://leetcode-cn.com/problems/sum-of-left-leaves/solution/javadi-gui-yu-die-dai-shi-xian-si-lu-by-ggb2312/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;
    impl Solution {
        pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            Self::dfs_sum(root, false)
        }

        fn dfs_sum(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            if let Some(root) = root {
                let root = root.borrow();

                if is_left && root.left.is_none() && root.right.is_none() {
                    return root.val;
                }
                Self::dfs_sum(root.left.clone(), true) + Self::dfs_sum(root.right.clone(), false)
            } else {
                0
            }
        }
    }
}

pub mod solution_bfs {
    use super::*;

    /// # 思路
    ///
    /// bfs遍历判断左叶子结点找出sum
    ///
    /// 如何找出左叶子结点？
    ///
    /// 以`root.left`作root判断下一个left,right是否为空
    ///
    /// ## Submission
    ///
    /// date=20200808, mem=2.1, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/95895570/
    ///
    /// author=navyd
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;
    impl Solution {
        pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut sum = 0;
            if let Some(root) = root {
                let mut stack = std::collections::VecDeque::new();
                stack.push_back(root);
                while !stack.is_empty() {
                    for _ in 0..stack.len() {
                        let root = stack.pop_front().unwrap();
                        let root = root.borrow();
                        if let Some(left) = root.left.clone() {
                            stack.push_back(left.clone());
                            let left = left.borrow();
                            if left.left.is_none() && left.right.is_none() {
                                sum += left.val;
                            }
                        }

                        if let Some(right) = root.right.clone() {
                            stack.push_back(right);
                        }
                    }
                }
            }
            sum
        }
    }
}
