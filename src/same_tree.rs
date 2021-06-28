/// 总结：
///
/// 1. 用java表示时出现不知道如何正确退出递归条件，用`if (p == null && q != null || (p != null && q == null))`
/// 可以更精简`if p == null && q == null else if q == null || p == null`
///
/// 2. rust的引用比较还是不熟，PartialEq的性质第一次知道
use std::cell::RefCell;
use std::rc::Rc;

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

pub mod solution_recursive {
    use super::*;
    /// # 思路
    ///
    /// 递归比较每个元素是否一样，退出递归条件 p,q为none时true，其中一个none时为false
    ///
    /// ## Submissions
    ///
    /// date=20200706, mem=2.1 mem_beats=33.33, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/362712136/
    ///
    /// author=GuiYom, references=https://leetcode.com/problems/same-tree/discuss/428859/Rust-0-ms-iterative-recursive-and-native-solutions
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N log N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn is_same_tree(
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            Self::compare(&p, &q)
        }

        fn compare(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    p.val == q.val
                        && (Self::compare(&p.left, &q.left) && Self::compare(&p.right, &q.right))
                }
                (None, None) => true,
                _ => false,
            }
        }
    }
}

pub mod solution_derivable {
    use super::*;

    /// # 思路
    ///
    /// 利用rust`std::cmp::PartialEq`的derivable性质，比较所有字段是否相等
    ///
    /// ## Submissions
    ///
    /// date=20200706, mem=2.1 mem_beats=47.62, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/362717451/
    ///
    /// author=GuiYom, references=https://leetcode.com/problems/same-tree/discuss/428859/Rust-0-ms-iterative-recursive-and-native-solutions
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N log N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn is_same_tree(
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            p == q
        }
    }
}
