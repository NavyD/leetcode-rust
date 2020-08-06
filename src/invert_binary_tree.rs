//! Invert a binary tree.
//!
//! Example:
//!
//! Input:
//! ```ignore
//!      4
//!    /   \
//!   2     7
//!  / \   / \
//! 1   3 6   9
//! ```
//! Output:
//! ```ignore
//!      4
//!    /   \
//!   7     2
//!  / \   / \
//! 9   6 3   1
//! ```
//! Trivia:
//! This problem was inspired by this original tweet by Max Howell:
//!
//! Google: 90% of our engineers use the software you wrote (Homebrew), but you can’t invert a binary tree on a whiteboard so f*** off.

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
    /// 通过翻转root的left, right结点就可以翻转整颗树
    /// 
    /// ![](https://pic.leetcode-cn.com/f9e06159617cbf8372b544daee37be70286c3d9b762c016664e225044fc4d479-226_%E8%BF%AD%E4%BB%A3.gif)
    /// 
    /// ## Submission
    /// 
    /// date=20200806, mem=2.1, mem_beats=32.5, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/376849754/
    /// 
    /// author=王尼玛, references=https://leetcode-cn.com/problems/invert-binary-tree/solution/dong-hua-yan-shi-liang-chong-shi-xian-226-fan-zhua/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(root) = root.clone() {
                let mut deque = std::collections::VecDeque::new();
                deque.push_back(root);
                while !deque.is_empty() {
                    for _ in 0..deque.len() {
                        let root = deque.pop_front().unwrap();
                        let mut root = root.borrow_mut();
                        // swap
                        let tmp = root.left.clone();
                        root.left = root.right.clone();
                        root.right = tmp;
                        if let Some(left) = root.left.clone() {
                            deque.push_back(left);
                        }
                        if let Some(right) = root.right.clone() {
                            deque.push_back(right);
                        }
                    }
                }
            }
            root
        }
    }
}

pub mod solution_dfs {
    use super::*;

    /// # 思路
    /// 
    /// 通过dfs递归swap
    /// 
    /// ![](https://pic.leetcode-cn.com/0f91f7cbf5740de86e881eb7427c6c3993f4eca3624ca275d71e21c5e3e2c550-226_2.gif)
    /// 
    /// 先中后序都可以
    /// 
    /// ## Submission
    /// 
    /// date=20200806, mem=2.1, mem_beats=15, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/376864608/
    /// 
    /// author=王尼玛, references=https://leetcode-cn.com/problems/invert-binary-tree/solution/dong-hua-yan-shi-liang-chong-shi-xian-226-fan-zhua/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(root) = root {
                let mut root_ref = root.borrow_mut();
                let tmp = root_ref.left.clone();
                root_ref.left = root_ref.right.clone();
                root_ref.right = tmp;
                Self::invert_tree(root_ref.left.clone());
                Self::invert_tree(root_ref.right.clone());
                return Some(root.clone())
            } else {
                None
            }
        }
    }
}