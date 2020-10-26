//! 为何需要同时用中序与前序才能确定二叉树
//! 前序无法确定左右子树，要加上中序遍历的数组来确定
//!
//! 如何在中序找左右节点
use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_recursive {
    use super::*;
    use std::collections::HashMap;

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
    /// 终止条件：当p_start>p_end。表示当前前序节点都完成迭代，或当前子树已完成
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
    pub struct Solution;

    impl Solution {
        pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.len() == 0 {
                return None;
            }
            let mut indexes = HashMap::new();
            inorder.iter().enumerate().for_each(|(i, v)| {
                indexes.insert(*v, i);
            });
            Self::helper(
                &preorder,
                0,
                preorder.len() - 1,
                &inorder,
                0,
                inorder.len() - 1,
                &mut indexes,
            )
        }

        fn helper(
            preorder: &Vec<i32>,
            p_start: usize,
            p_end: usize,
            inorder: &Vec<i32>,
            i_start: usize,
            i_end: usize,
            indexes: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if p_start > p_end {
                return None;
            }
            let mut root = TreeNode::new(preorder[p_start]);
            // let mut root =
            let i_root_idx = *indexes.get(&root.val).unwrap();
            let left_nums = i_root_idx - i_start;
            root.left = Self::helper(
                preorder,
                p_start + 1,
                p_start + left_nums,
                inorder,
                i_start,
                i_root_idx - 1,
                indexes,
            );
            root.right = Self::helper(
                preorder,
                p_start + left_nums + 1,
                p_end,
                inorder,
                i_root_idx + 1,
                i_end,
                indexes,
            );
            Some(Rc::new(RefCell::new(root)))
        }
    }
}
