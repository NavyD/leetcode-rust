use crate::helper::*;

pub mod solution_dfs {
    use super::*;

    /// # 思路
    ///
    /// hashmap保存2个以上的值，
    /// 找出时用两次遍历先找max count，再用count去找val
    ///
    /// ## Submission
    ///
    /// date=20200809, mem=2.9, mem_beats=100, runtime=4， runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/96161879/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    use std::collections::HashMap;

    impl Solution {
        pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut counts = HashMap::new();
            Self::dfs_counts(root, &mut counts);
            let mut max_count = 0;
            for (_, v) in &counts {
                if max_count < *v {
                    max_count = *v;
                }
            }

            let mut res = vec![];
            for (k, v) in &counts {
                if max_count == *v {
                    res.push(*k);
                }
            }
            res
        }

        fn dfs_counts(root: Option<Rc<RefCell<TreeNode>>>, counts: &mut HashMap<i32, u32>) {
            if let Some(root) = root {
                let root = root.borrow();
                *counts.entry(root.val).or_insert(0) += 1;
                Self::dfs_counts(root.left.clone(), counts);
                Self::dfs_counts(root.right.clone(), counts);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basic_solution_dfs() {
            use solution_dfs::Solution;
            assert_eq!(vec![2], Solution::find_mode(convert_tree("[1,null,2,2]")));
            assert_eq!(vec![1], Solution::find_mode(convert_tree("[1,1]")));
        }
    }
}

/// 总结
///
/// 有一个初始版，没有弄清该方法的原理，一直都无法通过，反复在中序parent=last_root,
/// pre是什么纠结，调试发现` cur_count: &mut u32, max_count: &mut u32,`有无用引用
/// 有区别，很难受，[1,1]这个测试无法通过。
///
/// ```ignore
/// fn inorder(
///     root: Option<Rc<RefCell<TreeNode>>>,
///     parent: Option<Rc<RefCell<TreeNode>>>,
///     cur_count: &mut u32,
///     max_count: &mut u32,
///     modes: &mut Vec<i32>,
/// ) {
///     if let Some(root) = root {
///         Self::inorder(
///             root.borrow().left.clone(),
///             Some(root.clone()),
///             cur_count,
///             max_count,
///             modes,
///         );
///     }
/// }
/// ```
///
/// 一直都没有数组转tree的方法调试，用了点时间写了个helper，
/// 后面用本地debug才发现什么区别的，靠
///
/// rust还是很生疏的
pub mod solution_dfs_follow_up {
    use super::*;

    /// # 思路
    ///
    /// 递归中比较前驱节点pre与当前节点，找cur_count, max_count，表示一个val出现次数
    ///
    /// - 如果`pre_node.val==cur_node.val`则cur_count += 1 表示还是之前的val
    /// - 如果`pre_node.val!=cur_node.val`则cur_count = 1 表示新开始一个val
    ///
    /// 在新node更新时当
    ///
    /// - `cur_count == max_count`：表示是不同的值出现数量一样，加入modes中。由于
    /// 题目定义相等值必然会连续出现在子树中，一旦是不同值则会中断cur_count
    /// - `cur_count > max_count`：表示之前的modes不正确，当前val出现更多
    ///
    /// ## 注意
    ///
    /// 该方法主要还是inorder，其中pre, cur_count, max_count都是引用，在整个
    /// 递归过程中不能被回溯时值覆盖已改变的值。
    ///
    /// 如果是用传值的方式则会导致回溯时之前max_count, pre回到之前状态
    ///
    /// ```ignore
    /// fn inorder(
    ///     root: Option<Rc<RefCell<TreeNode>>>,
    ///     modes: &mut Vec<i32>,
    ///     // 传值 不可用！！
    ///     pre: Option<Rc<RefCell<TreeNode>>>,
    ///     cur_count: u32,
    ///     max_count: u32,
    /// ) {
    /// ```
    ///
    /// 如果要用java的方式对primitive传值，可在struct中作为字段保证不会回溯更改
    ///
    /// ## Submissions
    ///
    /// date=20200811, mem=3.1, mem_beats=100, runtime=4, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/96889428/
    ///
    /// date=20200812, mem=3, mem_beats=100, runtime=4, runtime_beats=66.67, url=https://leetcode-cn.com/submissions/detail/97226821/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)。如果树结点val在后面出现一次多一次，导致modes.clear()频繁更新，用时更多
    /// - 空间：O(log N)
    ///
    /// ## 参考
    ///
    /// - [Java 4ms Beats 100% Extra O(1) solution - No Map](https://leetcode.com/problems/find-mode-in-binary-search-tree/discuss/98100/Java-4ms-Beats-100-Extra-O(1)-solution-No-Map)
    /// - [二叉搜索树中的众数](https://leetcode-cn.com/problems/find-mode-in-binary-search-tree/solution/er-cha-sou-suo-shu-zhong-de-zhong-shu-by-junstat/)
    pub struct Solution;

    impl Solution {
        pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut modes = vec![];
            Self::find_mode_by_dfs_inorder(root, &mut None, &mut 0, &mut 0, &mut modes);
            modes
        }

        fn find_mode_by_dfs_inorder(
            root: Option<Rc<RefCell<TreeNode>>>,
            pre: &mut Option<Rc<RefCell<TreeNode>>>,
            cur_count: &mut u32,
            max_count: &mut u32,
            modes: &mut Vec<i32>,
        ) {
            if let Some(root) = root {
                // pre在第一个叶子结点left时为None，
                // 后面一直都是上个root
                Self::find_mode_by_dfs_inorder(
                    root.borrow().left.clone(),
                    pre,
                    cur_count,
                    max_count,
                    modes,
                );
                let root_val = root.borrow().val;
                // 连续的相等的node
                if let Some(pre) = pre {
                    if pre.borrow().val == root_val {
                        *cur_count += 1;
                    }
                    // 新node
                    else {
                        *cur_count = 1;
                    }
                }
                if cur_count == max_count {
                    modes.push(root_val);
                } else if cur_count > max_count {
                    modes.clear();
                    modes.push(root_val);
                    *max_count = *cur_count;
                }
                *pre = Some(root.clone());
                Self::find_mode_by_dfs_inorder(
                    root.borrow().right.clone(),
                    pre,
                    cur_count,
                    max_count,
                    modes,
                );
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basic_solution_dfs_follow_up() {
            use solution_dfs_follow_up::Solution;
            assert_eq!(vec![2], Solution::find_mode(convert_tree("[1,null,2,2]")));
            assert!(is_included_vec(
                &vec![1, 2],
                &Solution::find_mode(convert_tree("[1,2]"))
            ));
            assert_eq!(vec![1], Solution::find_mode(convert_tree("[1,1]")));
            assert_eq!(vec![-2], Solution::find_mode(convert_tree("[-2,-2,-2]")));
        }
    }
}
