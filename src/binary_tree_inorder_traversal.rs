use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_recursive {
    use super::*;

    /// ### Submissions
    ///
    /// date=20200812, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/97279521/
    ///
    /// date=20201004, mem=2.1, mem_beats=8.16, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/113371626/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)。不考虑结果vals
    pub struct Solution;

    impl Solution {
        pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            Self::inorder(root, &mut vals);
            vals
        }

        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(root) = root {
                let root = root.borrow();
                Self::inorder(root.left.clone(), vals);
                vals.push(root.val);
                Self::inorder(root.right.clone(), vals);
            }
        }
    }
}

/// 第1次
///
/// 只记得通用递归，stack忘记了
///
/// 20201005
///
/// 对如何处理外部循环root=root.right的方式不清楚，如何结束外部循环
pub mod solution_stack {
    use super::*;

    /// # 思路
    ///
    /// 这里有一个更简洁的方式
    ///
    /// ```rust,ignore
    /// // date=20201005, mem=1.9, mem_beats=89.80, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/113381820/
    /// pub fn inorder_traversal1(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    ///     let (mut stack, mut vals) = (vec![], vec![]);
    ///     while root.is_some() || !stack.is_empty() {
    ///         // push left
    ///         if let Some(node) = root {
    ///             root = node.borrow().left.clone();
    ///             stack.push(node);
    ///         } else if let Some(node) = stack.pop() {
    ///             vals.push(node.borrow().val);
    ///             // right as root
    ///             root = node.borrow().right.clone();
    ///         }
    ///     }
    ///     vals
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [memory/1924](https://leetcode-cn.com/submissions/api/detail/94/rust/memory/1924/)
    ///
    /// ### Submissions
    ///
    /// date=20200812, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/97285664/
    ///
    /// date=20200813, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/97601369/
    ///
    /// date=20200818, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/99442141/
    ///
    /// date=20201005, mem=2, mem_beats=20.41, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/113379004/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let (mut stack, mut vals) = (vec![], vec![]);
            while root.is_some() || !stack.is_empty() {
                // push left
                while let Some(node) = root.clone() {
                    root = node.borrow().left.clone();
                    stack.push(node);
                }
                // pop left
                let node = stack.pop().unwrap();
                vals.push(node.borrow().val);
                // right as new root
                root = node.borrow().right.clone();
            }
            vals
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_recursive::Solution::inorder_traversal);
        test(solution_stack::Solution::inorder_traversal);
    }

    fn test<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>>(func: F) {
        assert_eq!(vec![1, 3, 2], func(btree![1, null, 2, 3]));
        let res: Vec<i32> = vec![];
        assert_eq!(res, func(btree![]));
        assert_eq!(vec![1], func(btree![1]));
        assert_eq!(vec![2, 1], func(btree![1, 2]));
        assert_eq!(vec![1, 2], func(btree![1, null, 2]));
    }
}
