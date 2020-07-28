//! Given a binary tree, determine if it is height-balanced.
//!
//! For this problem, a height-balanced binary tree is defined as:
//!
//! a binary tree in which the left and right subtrees of every node differ in height by no more than 1.
//!
//!  
//!
//! Example 1:
//!
//! Given the following tree [3,9,20,null,null,15,7]:
//!
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! Return true.
//!
//! Example 2:
//!
//! Given the following tree [1,2,2,3,3,null,null,4,4]:
//!
//!        1
//!       / \
//!      2   2
//!     / \
//!    3   3
//!   / \
//!  4   4
//! Return false.

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

pub mod solution_top_to_bottom {
    use super::*;

    /// # 思路
    ///
    /// 如果一个树有`abs(height(p.left) - height(p.right)) > 1`则不是平衡的
    ///
    /// 只要计算某个节点的left,right高度差即可，设height(p)为p节点的高度
    /// 则有`height(p) = max(height(left), height(right)) + 1`
    ///
    /// 只要递归比较root.left与right高度差即可
    /// 
    /// ## Submission
    /// 
    /// date=20200728, mem=2.7, mem_beats=2.7, runtime=4, runtime_beats=33.33, url=https://leetcode.com/submissions/detail/372654896/
    /// 
    /// author=Krahets, references=https://leetcode-cn.com/problems/balanced-binary-tree/solution/balanced-binary-tree-di-gui-fang-fa-by-jin40789108/
    /// 
    /// ## 复杂度
    ///
    /// - 时间：height要计算高度为O(log N)，is_balanced为遍历O(N)则为O(N log N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(root) = root {
                let root = root.borrow();
                (Self::height(root.left.to_owned()) 
                    - Self::height(root.right.to_owned()))
                    .abs() <= 1
                    && Self::is_balanced(root.left.to_owned())
                    && Self::is_balanced(root.right.to_owned())
            } else {
                true
            }
        }

        fn height(root: Option<Rc<RefCell<TreeNode>>>) -> isize {
            if let Some(root) = root {
                let root = root.borrow();
                Self::height(root.left.to_owned()).max(Self::height(root.right.to_owned())) + 1
            } else {
                0
            }
        }
    }
}
