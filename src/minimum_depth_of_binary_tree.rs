use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// 注意不能用下面的match，导致(Some, Some)的模式会被(Some, _)执行
/// 
/// ```rust,ignore
/// match (root.left.clone(), root.right.clone()) {
///     (None, None) => return level,
///     (Some(left), _) => queue.push_back(left),
///     (_, Some(right)) => queue.push_back(right),
/// }
/// ```
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
    /// date=20200730, mem=3.4, mem_beats=5.88, runtime=28, runtime_beats=5.66, url=https://leetcode-cn.com/submissions/detail/116940727/
    /// 
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut level = 0;
            if let Some(root) = root {
                level += 1;
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(root);
                while !queue.is_empty() {
                    for _ in 0..queue.len() {
                        let root = queue.pop_front().unwrap();
                        let root = root.borrow();
                        if root.left.is_none() && root.right.is_none() {
                            return level;
                        }
                        if let Some(left) = root.left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = root.right.clone() {
                            queue.push_back(right);
                        }
                    }
                    level += 1;
                }
            }
            level
        }
    }
}

/// 没有想清楚递归条件，一直返回min(left,right)，没有考虑当lef或right为空时，
/// min(left,right)一直为0，使得下面这种结构的树depth=1
///
/// ```ignore
///         1
///          \
///           2
/// ```
///
pub mod solution_dfs {
    use super::*;

    /// # 思路
    ///
    /// 在dfs递归回溯时计算每个node的深度，比较取最小值给上个节点直到root得到最小深度
    ///
    /// 注意在left, right一个节点为None时其min depth是非None depth+1，
    /// 都不是None时取min
    ///
    /// 叶子节点的定义是左孩子和右孩子都为 null 时叫做叶子节点
    ///
    /// - 当 root 节点左右孩子都为空时，返回 1
    /// - 当 root 节点左右孩子有一个为空时，返回不为空的孩子节点的深度
    /// - 当 root 节点左右孩子都不为空时，返回左右孩子较小深度的节点值
    ///
    /// 参考：
    ///
    /// - [My 4 Line java solution](https://leetcode.com/problems/minimum-depth-of-binary-tree/discuss/36045/My-4-Line-java-solution)
    /// - [二叉树的最小深度-理解递归结束条件](https://leetcode-cn.com/problems/minimum-depth-of-binary-tree/solution/li-jie-zhe-dao-ti-de-jie-shu-tiao-jian-by-user7208/)
    ///
    /// ## Submission
    ///
    /// date=20200730, mem=2.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/373460334/
    ///
    /// date=20201019, mem=3.7, mem_beats=5.88, runtime=32, runtime_beats=5.66, url=https://leetcode-cn.com/submissions/detail/116902948/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            root.map_or(0, |root| {
                let root = root.borrow();
                let left_depth = Self::min_depth(root.left.clone());
                let right_depth = Self::min_depth(root.right.clone());
                if left_depth == 0 || right_depth == 0 {
                    left_depth + right_depth + 1
                } else {
                    left_depth.min(right_depth) + 1
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::min_depth);
        test(solution_bfs::Solution::min_depth);
    }

    fn test<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> i32>(func: F) {
        assert_eq!(func(btree![3, 9, 20, null, null, 15, 7]), 2);
        assert_eq!(func(btree![2, null, 3, null, 4, null, 5, null, 6]), 5);
        assert_eq!(func(btree![1,2,3,4,5]), 2);
    }
}
