//! 如何翻转整个二叉树
//!
//! 只要翻转root.left,right，并应用到后续所有子树，即可完成翻转

use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// 20201015
///
/// 卡在如何交换left,right与push顺序
pub mod solution_bfs {
    use super::*;

    /// # 思路
    ///
    /// 一层层的换节点
    ///
    /// 通过翻转root的left, right结点就可以翻转整颗树
    ///
    /// ![](https://pic.leetcode-cn.com/f9e06159617cbf8372b544daee37be70286c3d9b762c016664e225044fc4d479-226_%E8%BF%AD%E4%BB%A3.gif)
    ///
    /// 下面的代码意思是一致的，但是多了不必要的`for _ in`，由于queue的FIFO特性，不断的push_back使一层连着一层，而不是
    /// 用`for _ in 0..len`分开，while pop_front确保一层的会连续处理，同时连续处理下一层
    ///
    /// ```rust,ignore
    /// pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    ///     if let Some(root) = root.clone() {
    ///         let mut deque = std::collections::VecDeque::new();
    ///         deque.push_back(root);
    ///         while !deque.is_empty() {
    ///             for _ in 0..deque.len() {
    ///                 let root = deque.pop_front().unwrap();
    ///                 let mut root = root.borrow_mut();
    ///                 // swap
    ///                 let tmp = root.left.clone();
    ///                 root.left = root.right.clone();
    ///                 root.right = tmp;
    ///                 if let Some(left) = root.left.clone() {
    ///                     deque.push_back(left);
    ///                 }
    ///                 if let Some(right) = root.right.clone() {
    ///                     deque.push_back(right);
    ///                 }
    ///             }
    ///         }
    ///     }
    ///     root
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [动画演示 两种实现 226. 翻转二叉树](https://leetcode-cn.com/problems/invert-binary-tree/solution/dong-hua-yan-shi-liang-chong-shi-xian-226-fan-zhua/)
    ///
    /// ## Submission
    ///
    /// date=20200806, mem=2.1, mem_beats=32.5, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/376849754/
    ///
    /// date=20201014, mem=1.9, mem_beats=84.31, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115684571/
    ///
    /// date=20201015, mem=2, mem_beats=54.9, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115928496/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            root.map(|root| {
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(root.clone());
                while let Some(root) = queue.pop_front() {
                    let mut root = root.borrow_mut();
                    // swap left and right
                    let tmp = root.left.take();
                    root.left = root.right.take();
                    root.right = tmp;
                    // add sub nodes
                    if let Some(left) = root.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = root.right.clone() {
                        queue.push_back(right);
                    }
                }
                root
            })
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
    /// rust在回溯操作上不能swap，所有权被收回，只能在递归前序swap
    ///
    /// 参考：
    ///
    /// - [动画演示 两种实现 226. 翻转二叉树](https://leetcode-cn.com/problems/invert-binary-tree/solution/dong-hua-yan-shi-liang-chong-shi-xian-226-fan-zhua/)
    ///
    /// ## Submission
    ///
    /// date=20200806, mem=2.1, mem_beats=15, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/376864608/
    ///
    /// date=20201014, mem=1.9, mem_beats=94.12, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115672595/
    ///
    /// date=20201015, mem=2.1, mem_beats=5.88, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115929298/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            root.map(|root| {
                let mut root_ref = root.borrow_mut();
                // swap left and right
                let tmp = root_ref.left.take();
                root_ref.left = root_ref.right.take();
                root_ref.right = tmp;
                // sub tree
                Self::invert_tree(root_ref.left.clone());
                Self::invert_tree(root_ref.right.clone());
                root.clone()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::invert_tree);
        test(solution_bfs::Solution::invert_tree);
    }

    fn test<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>>(func: F) {
        assert_eq!(
            btree![4, 7, 2, 9, 6, 3, 1],
            func(btree![4, 2, 7, 1, 3, 6, 9])
        );
    }
}
