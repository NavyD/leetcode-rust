//! 如何找当p|q为root时，对应q|p为其子树的情况，要知道子树的情况

use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_dfs {
    use super::*;

    ///
    /// lowest common ancestor (LCA)
    ///
    /// 若 root 是 p, q 的 最近公共祖先 ，则只可能为以下情况之一：
    ///
    /// - p 和 q 在 root 的子树中，且分列 root 的 异侧（即分别在左、右子树中）
    /// - p = root ，且 q 在 root 的左或右子树中；
    /// - q = root ，且 p 在 root 的左或右子树中；
    ///
    /// 考虑通过递归对二叉树进行后序遍历，当遇到节点 p 或 q 时返回。从底至顶回溯，当节点 p,q 在节点 root 的异侧时，节点 root 即为最近公共祖先，则向上返回 root 。
    ///
    /// ### 递归条件
    ///
    /// 终止：当root==None或root==q|p时返回root为子结点
    ///
    /// 递归：后序
    ///
    /// 返回：
    ///
    /// - 当left与right为空时，表示没有找到p q返回None
    /// - 当都为Some时表示当前root是对应的LCA，返回非空的结点
    /// - 当left|right仅一个为空时，直接返回非空的right|left，当在后序找到left|right时会更新作为其结点作为LCA
    ///
    /// 注意：当仅一个为空时，如果pq的有一个不在树中，不应该返回子树结，应该返回null，但是leetcode测试是返回存在的结点
    ///
    /// ```rust,ignore
    /// // 可以被改进成后面的3行
    /// if left.is_none() && right.is_none() {
    ///     None
    /// } else if left.is_none() {
    ///     right
    /// } else if right.is_none() {
    ///     left
    /// } else {
    ///     Some(root.clone())
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [236. 二叉树的最近公共祖先（后序遍历 DFS ，清晰图解）](https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-tree/solution/236-er-cha-shu-de-zui-jin-gong-gong-zu-xian-hou-xu/)
    /// - [My Java Solution which is easy to understand](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/65226/My-Java-Solution-which-is-easy-to-understand)
    ///
    /// ### Submissions
    ///
    /// date=20201025, mem=4.6, mem_beats=100, runtime=4, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/118425007/
    ///
    /// date=20201025, mem=4.4, mem_beats=50, runtime=4, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/118661709/
    ///
    /// date=20201124, mem=4.5, mem_beats=20, runtime=8, runtime_beats=93, url=https://leetcode-cn.com/submissions/detail/125885807/
    /// 
    /// date=20201124, mem=4, mem_beats=76, runtime=4, runtime_beats=95, url=https://leetcode-cn.com/submissions/detail/133524720/ 
    pub struct Solution;

    impl Solution {
        pub fn lowest_common_ancestor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            fn _helper(
                root: Option<Rc<RefCell<TreeNode>>>,
                p_val: i32,
                q_val: i32,
            ) -> Option<Rc<RefCell<TreeNode>>> {
                if root.as_ref().map_or(true, |root| {
                    root.borrow().val == p_val || root.borrow().val == q_val
                }) {
                    return root;
                }
                let root = root.unwrap();
                let left = _helper(root.borrow().left.clone(), p_val, q_val);
                let right = _helper(root.borrow().right.clone(), p_val, q_val);
                if left.is_none() {
                    right
                } else if right.is_none() {
                    left
                } else {
                    Some(root.clone())
                }
            }
            _helper(root, p.unwrap().borrow().val, q.unwrap().borrow().val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::lowest_common_ancestor);
    }

    fn test<F>(func: F)
    where
        F: Fn(
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>>,
    {
        let val = func(
            btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
            btree![5],
            btree![1],
        )
        .unwrap()
        .borrow()
        .val;
        assert_eq!(val, 3);

        let val = func(
            btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
            btree![5],
            btree![4],
        )
        .unwrap()
        .borrow()
        .val;
        assert_eq!(val, 5);
    }
}
