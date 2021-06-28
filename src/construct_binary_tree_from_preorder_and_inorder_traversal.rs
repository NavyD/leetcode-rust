//! 为何需要同时用中序与前序才能确定二叉树
//! 前序无法确定左右子树，要加上中序遍历的数组来确定
//!
//! 如何在中序找左右节点
use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_recursive {
    use super::*;

    /// 前序确定根节点，中序确定根节点对应的的左右子树。
    ///
    /// 设定根节点前序是p_start，中序对应开始节点为i_start，则在中序inorder中找到preorder[p_start]对应的下标i_root_idx，
    /// i_start..i_root_idx都是preorder[p_start]的左子树，
    ///
    /// 如何确定前序左子树的范围。通过数i_root_idx左边的数量
    ///
    /// 如何确定右子树，设i_end为最后inorder下标，当用i_root_idx分开时，i_root_idx..i_end就是右子树
    ///
    /// ### 递归
    ///
    /// 终止条件：当p_start>p_end。表示当前前序节点都完成迭代，当前子树已完成
    ///
    /// 递归工作：
    ///
    /// - 构造root找到p_start对应的i_root_idx
    /// - 计算left_nums表示i_root_idx - i_start对应有多少左子树元素。
    /// - 递归构造root.left, right
    ///
    /// 返回：返回root节点
    ///
    /// 参考：
    ///
    /// - [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by--22/)
    ///
    /// ### Submissions
    ///
    /// date=20201026, mem=2.7, mem_beats=38.89, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/118655685/
    ///
    /// date=20201027, mem=2.6, mem_beats=61.11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/118921403/
    ///
    pub struct Solution;

    impl Solution {
        pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            use std::collections::HashMap;
            fn _helper(
                preorder: &[i32],
                p_start: usize,
                p_end: usize,
                inorder: &[i32],
                i_start: usize,
                i_end: usize,
                i_indexes: &HashMap<i32, usize>,
            ) -> Option<Rc<RefCell<TreeNode>>> {
                if p_start > p_end {
                    return None;
                }
                let mut root = TreeNode::new(preorder[p_start]);
                let i_root_idx = i_indexes.get(&root.val).unwrap();
                let left_len = i_root_idx - i_start;

                root.left = if *i_root_idx == 0 {
                    // fixed: i_root_idx - 1
                    // 存在 attempt to subtract with overflow error for local test
                    None
                } else {
                    _helper(
                        preorder,
                        // 排除当前root节点
                        p_start + 1,
                        p_start + left_len,
                        inorder,
                        i_start,
                        // 排除当前root节点
                        i_root_idx - 1,
                        i_indexes,
                    )
                };

                root.right = _helper(
                    preorder,
                    // 排除当前root节点
                    p_start + left_len + 1,
                    p_end,
                    inorder,
                    // 排除当前root节点
                    i_root_idx + 1,
                    i_end,
                    i_indexes,
                );
                Some(Rc::new(RefCell::new(root)))
            }
            if preorder.is_empty() {
                return None;
            }
            let mut indexes = HashMap::new();
            inorder.iter().enumerate().for_each(|(i, v)| {
                indexes.insert(*v, i);
            });
            _helper(
                &preorder,
                0,
                preorder.len() - 1,
                &inorder,
                0,
                inorder.len() - 1,
                &indexes,
            )
        }
    }
}

pub mod solution_recursive_slice {
    use super::*;

    /// 为何要用root_idx切分preorder：分割的inorder不能确定具体子树。划分后的right子树不能用与left下个
    /// 元素作为right的root，而是要对应到left子树数量后下个preorder才是right的root。而root_idx就是
    /// left元素的数量
    ///
    /// ### Submission
    ///
    /// date=20201220, mem=2.6, mem_beats=63, runtime=4, runtime_beats=72, url=https://leetcode-cn.com/submissions/detail/118908310/
    ///
    /// date=20201220, mem=2.6, mem_beats=63, runtime=4, runtime_beats=72, url=
    pub struct Solution;

    impl Solution {
        pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            fn _helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
                if inorder.is_empty() {
                    None
                } else {
                    let mut root = TreeNode::new(preorder[0]);
                    let root_idx = inorder.iter().position(|e| *e == root.val).unwrap();
                    let next_root_idx = root_idx + 1;
                    // inorder排除当前root preorder传入对应root节点的数量
                    root.left = _helper(&preorder[1..next_root_idx], &inorder[..root_idx]);
                    root.right = _helper(&preorder[next_root_idx..], &inorder[next_root_idx..]);
                    Some(Rc::new(RefCell::new(root)))
                }
            }
            _helper(&preorder, &inorder)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_recursive::Solution::build_tree);
        test(solution_recursive_slice::Solution::build_tree);
    }

    fn test<F: Fn(Vec<i32>, Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>>(func: F) {
        assert_eq!(
            func(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            btree![3, 9, 20, null, null, 15, 7]
        );
    }
}
