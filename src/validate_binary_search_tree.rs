//! 如何递归比较root,left,right的大小

use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// 在第一次写时，只想到用root.val与left,right比较，没有更多思考，递归条件何时返回，如何处理都没有理清
///
/// ```rust,ignore
/// pub fn is_valid_bst_error(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
///     if let Some(root) = root {
///         let root = root.borrow();
///         if let Some(left) = root.left.clone() {
///             return if left.borrow().val < root.val {
///                 Self::is_valid_bst(Some(left))
///             } else {
///                 false
///             };
///         }
///         if let Some(right) = root.right.clone() {
///             return if right.borrow().val > root.val {
///                 Self::is_valid_bst(Some(right))
///             } else {
///                 false
///             };
///         }
///     }
///     true
/// }
/// ```
///
/// 第二次没有任何提高。思路还是递归的比较root与left,right节点
///
/// ```ignore
/// pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
///     root.map_or(true, |root| {
///         let root = root.borrow();
///
///         if let Some(left) = &root.left {
///             if root.val <= left.borrow().val {
///                 return false;
///             }
///         }
///
///         if let Some(right) = &root.right {
///             if root.val >= right.borrow().val {
///                 return false;
///             }
///         }
///         is_valid_bst(root.left.clone()) && is_valid_bst(root.right.clone())
///     })
/// }
/// ```
///
/// 问题在于没有考虑中序比较大小 或 递归时限制当前节点的大小：如下面的图片中：6所在的节点应该
/// 存在满足： 6 < 15 && 6 > 10。6作为root:10的右子树节点应该比root大，显然6是不合法的，
/// 所以上面的递归思路不对
///
/// ![](https://pic.leetcode-cn.com/1602255638-CdMLrU-image.png)
///
/// 参考：
///
/// [三种解决方式，两种击败了100%的用户](https://leetcode-cn.com/problems/validate-binary-search-tree/solution/san-chong-jie-jue-fang-shi-liang-chong-ji-bai-liao/)
pub mod solution_recursive {
    use super::*;

    /// # 思路
    ///
    /// 使用递归与root对应范围限制检查
    ///
    /// 注意：lower,upper必须比i32更大，使用i64。对一个子树，最小最大的val可用root.val确定
    ///
    /// 参考：
    ///
    /// - [三种解决方式，两种击败了100%的用户](https://leetcode-cn.com/problems/validate-binary-search-tree/solution/san-chong-jie-jue-fang-shi-liang-chong-ji-bai-liao/)
    /// - [验证二叉搜索树](https://leetcode-cn.com/problems/validate-binary-search-tree/solution/yan-zheng-er-cha-sou-suo-shu-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20201124, mem=3, mem_beats=43, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/125867022/
    pub struct Solution;

    impl Solution {
        pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            fn helper(root: Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
                root.map_or(true, |root| {
                    let root = root.borrow();
                    let val = root.val as i64;
                    val > lower
                        && val < upper
                        && helper(root.left.clone(), lower, val)
                        && helper(root.right.clone(), val, upper)
                })
            }
            helper(root, i64::MIN, i64::MAX)
        }
    }
}

pub mod solution_dfs {
    use super::*;

    /// # 思路
    ///
    /// 要验证二叉搜索树是有序的，只要像数组一样检查是否满足：前一个值 < 当前值。而中序遍历是有序的
    ///
    /// 中序遍历时，判断当前节点是否大于中序遍历的前一个节点，如果大于，说明满足 BST，继续遍历；否则直接返回 false。
    ///
    /// 注意`pre_val: std::i64::MIN`是比`root.val: i32`更小的，不能设为`std::i32::MIN`
    ///
    /// 中序node与pre_node每一步对应关系：
    ///
    /// ```ignore
    /// node:       left -> root -> right
    /// pre_node:   None -> left -> root
    /// ```
    ///
    /// - 当root=None时终止返回true。单root一定是有序的
    /// - 当left子树是无序是返回false
    /// - 如果root.val <= pre_val则返回false，否则更新pre_val=root.val
    /// - 返回right子树结果
    ///
    /// 参考：
    ///
    /// - [中序遍历轻松拿下，🤷‍♀️必须秒懂！](https://leetcode-cn.com/problems/validate-binary-search-tree/solution/zhong-xu-bian-li-qing-song-na-xia-bi-xu-miao-dong-/)
    ///
    /// ### Submissions
    ///
    /// date=20201015, mem=3.1, mem_beats=5, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115899425/
    ///
    /// date=20201016, mem=3.2, mem_beats=7.41, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/116145829/
    ///
    /// date=20201124, mem=3.1, mem_beats=10, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/125872012/
    pub struct Solution;
    impl Solution {
        pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            fn helper(root: Option<Rc<RefCell<TreeNode>>>, pre_val: &mut Option<i32>) -> bool {
                root.map_or(true, |root| {
                    let root = root.borrow();
                    if !helper(root.left.clone(), pre_val) {
                        return false;
                    }
                    if let Some(pre_val) = pre_val {
                        if *pre_val >= root.val {
                            return false;
                        }
                    }
                    *pre_val = Some(root.val);
                    helper(root.right.clone(), pre_val)
                })
            }
            helper(root, &mut None)
        }
    }
}

/// 20201016
/// 第二次 注意不能让`root = node.borrow().right.clone()`用下面代码替换，
/// 还有`while root.is_some() || !stack.is_empty()`条件不能忘记
///
/// ```rust, ignore
/// while !stack.is_empty() {
///     if let Some(right) = node.borrow().right.clone() {
///         stack.push(right);
///     }
///     // root = node.borrow().right.clone();
/// }
/// ```
pub mod solution_dfs_iterative {
    use super::*;

    /// # 思路
    ///
    /// 是[solution_dfs::Solution](../solution_dfs/struct.Solution.html)的迭代写法
    ///
    /// [inorder写法](../../binary_tree_inorder_traversal/solution_stack/struct.Solution.html)
    ///
    /// ### Submissions
    ///
    /// date=20201015, mem=3.1, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/408900690/
    ///
    /// date=20201016, mem=2.7, mem_beats=96.3, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/116151516/
    ///
    /// date=20201124, mem=2.8, mem_beats=86, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/125875758/
    pub struct Solution;
    impl Solution {
        pub fn is_valid_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            let mut stack = vec![];
            let mut pre_node = None as Option<Rc<RefCell<TreeNode>>>;
            while root.is_some() || !stack.is_empty() {
                if let Some(node) = root.clone() {
                    root = node.borrow().left.clone();
                    stack.push(node);
                } else if let Some(node) = stack.pop() {
                    if pre_node.map_or(false, |pre_node| node.borrow().val <= pre_node.borrow().val)
                    {
                        return false;
                    }
                    root = node.borrow().right.clone();
                    pre_node = Some(node);
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::is_valid_bst);
        test(solution_dfs_iterative::Solution::is_valid_bst);
        test(solution_recursive::Solution::is_valid_bst);
    }

    fn test<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> bool>(func: F) {
        assert!(func(btree![2, 1, 3]));
        assert!(func(btree![-2147483648]));
        assert!(!func(btree![5, 1, 4, null, null, 3, 6]));
        assert!(!func(btree![10, 5, 15, null, null, 6, 20]));
    }
}
