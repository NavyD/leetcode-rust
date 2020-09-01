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

/// 总结：
///
/// 没想到用两个结点同时比较

pub mod solution_recursion {
    use super::*;

    /// # 思路
    ///
    /// 递归比较同时两个子树left与right
    ///
    /// 注意比较left.left, right.right后要再比较left.right, right.left
    ///
    /// ![](https://pic.leetcode-cn.com/2449af8862537df2cbbc45a07764415c1a10769677c822fa271ea7447c8fa128-2.gif)
    ///
    /// ## Submissions
    ///
    /// date=20200709, mem=2.1, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/364095984/
    ///
    /// author=王尼玛, references=https://leetcode-cn.com/problems/symmetric-tree/solution/dong-hua-yan-shi-101-dui-cheng-er-cha-shu-by-user7/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N log N)
    /// - 空间：O(log N)
    ///
    pub struct Solution;

    impl Solution {
        pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(root) = root {
                Self::compare(&root.borrow().left, &root.borrow().right)
            } else {
                true
            }
        }

        fn compare(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (Some(left), Some(right)) => {
                    left.borrow().val == right.borrow().val
                        && Self::compare(&left.borrow().left, &right.borrow().right)
                        && Self::compare(&left.borrow().right, &right.borrow().left)
                }
                (None, None) => true,
                _ => false,
            }
        }
    }
}

pub mod solution_queue {
    use super::*;

    /// # 思路
    ///
    /// 用queue 一层一层比下去
    /// 
    /// ## Submissions
    /// 
    /// date=20200712, mem=2.1, mem_beats=64.71, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/365614242/
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let None = root {
                return true;
            }
            let root = root.unwrap();
            // let a = Rc::clone(root.);
            let mut queue = vec![];
            queue.push(root.borrow().left.clone());
            queue.push(root.borrow().right.clone());
            while let (Some(left), Some(right)) = (queue.pop(), queue.pop()) {
                match (left, right) {
                    (Some(left), Some(right)) => {
                        let left = left.borrow();
                        let right = right.borrow();
                        if left.val != right.val {
                            return false;
                        } else {
                            queue.push(left.left.clone());
                            queue.push(right.right.clone());
                            queue.push(left.right.clone());
                            queue.push(right.left.clone());
                        }
                    }
                    (None, None) => continue,
                    _ => return false,
                }
            }
            true
        }
    }
}
