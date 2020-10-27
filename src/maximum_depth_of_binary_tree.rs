//! 如何计算树深度
//!
//! dfs递归回溯统计或bfs层层遍历

use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_dfs {
    use super::*;
    /// # 思路
    ///
    /// 递归回溯时计算出深度
    ///
    /// - 当root=None时终止返回0
    /// - 计算root.left子树与root.right子树最大深度max_depth
    /// - root层尝试是子树深度`max_depth+1`
    ///
    /// ## Submissions
    ///
    /// date=20200713, mem=2.7, mem_beats=80.95, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/365857015/
    ///
    /// date=20201016, mem=2.5, mem_beats=92, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/116131087/
    ///
    /// date=20201018, mem=2.6, mem_beats=81.48, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/116632580/
    /// 
    /// date=20201027, mem=2.6， mem_beats=83.53, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/118930896/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)
    ///
    pub struct Solution;

    impl Solution {
        pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            root.map_or(0, |root| {
                Self::max_depth(root.borrow().left.clone())
                    .max(Self::max_depth(root.borrow().right.clone()))
                    + 1
            })
        }
    }
}

pub mod solution_bfs {
    use super::*;
    /// # 思路
    ///
    /// 一层一层向下计算
    ///
    /// 如何遍历平级结点？
    ///
    /// 注意在平级结点中存在后面子结点push_back，不能pop出子结点作为平级结点用，
    /// 应该用size个数限制
    ///
    /// 参考：
    ///
    ///- [My code of C++, Depth-first-search and Breadth-first-search](https://leetcode.com/problems/maximum-depth-of-binary-tree/discuss/34207/My-code-of-C%2B%2B-Depth-first-search-and-Breadth-first-search)
    ///
    /// ## Submissions
    ///
    /// date=20200713, mem=2.5， mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/365882616/
    ///
    /// date=20201016, mem=2.6， mem_beats=82.72, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/116132950/
    ///
    /// date=20201017, mem=2.3， mem_beats=97.53, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/116634700/
    /// 
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            root.map_or(0, |root| {
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(root);
                let mut level = 0;
                while !queue.is_empty() {
                    for _ in 0..queue.len() {
                        let root = queue.pop_front().unwrap();
                        let root = root.borrow();
                        if let Some(left) = root.left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = root.right.clone() {
                            queue.push_back(right);
                        }
                    }
                    level += 1;
                }
                level
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_bfs::Solution::max_depth);
        test(solution_dfs::Solution::max_depth);
    }

    fn test<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> i32>(func: F) {
        assert_eq!(3, func(btree![3, 9, 20, null, null, 15, 7]));
    }
}
