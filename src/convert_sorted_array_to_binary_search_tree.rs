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

pub mod solution_bfs {
    use super::*;

    /// # 思路
    ///
    /// 如何用有序数组构造平衡bst：
    ///
    /// 可以将数组已中点作为root，分为两个子结点，对子结点也用子数组的
    /// 中点。通过中序遍历的方式先root -> left -> right递归
    ///
    /// 如何解决mid-1 -> 0 - 1时的问题
    ///
    /// 当lo=0, hi=1时，mid=1，left=0, root=1，这时递归用lo=0, hi=mid-1=0
    /// 有mid=0, 下次递归时lo=0, hi=mid-1=-1使得usize runtime error
    ///
    /// ```rust, ignore
    /// fn dfs(nums: &Vec<i32>, lo: usize, hi: usize) -> Option<Rc<RefCell<TreeNode>>> {
    ///     if lo > hi {
    ///         return None;
    ///     }
    ///     let mid = (hi - lo + 1) / 2;
    ///     let root = Rc::new(RefCell::new(TreeNode::new(
    ///         nums[mid],
    ///     )));
    ///     root.borrow_mut().left = Self::dfs(nums, lo, mid);
    ///     root.borrow_mut().right = Self::dfs(nums, mid + 1, hi);
    ///     Some(root)
    /// }
    /// ```
    ///
    /// 应该用`lo>=hi`时返回，当lo=0,hi=1时用mid=0作为node，
    /// 此时如果hi=len-1, mid=2，hi=4 => lo=3, hi=4会使得right=nums[3]=5
    /// 计算错误，hi=len时正确，lo=3, hi=5, mid=4，right=nums[4]=9
    ///
    /// 还可以用slice代替下标[参考](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/discuss/326589/Rust-0ms-3.4MB)
    ///
    /// ```rust, ignore
    /// fn create_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    ///     let l = nums.len();
    ///     let mid = l / 2;
    ///     let mut node = TreeNode::new(nums[mid]);
    ///     if mid > 0 {
    ///         node.left = Solution::create_bst(&nums[0..mid]);
    ///     }
    ///     if mid + 1 < l {
    ///         node.right = Solution::create_bst(&nums[(mid + 1)..]);
    ///     }
    ///     Some(Rc::new(RefCell::new(node)))
    /// }
    /// ```
    ///
    /// ## Submission
    ///
    /// date=20200726, mem=3, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/371722183/
    ///
    /// author=petersmith7519, references=https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/discuss/509647/Rust-Solution-0ms-2.9MB
    ///
    /// author=
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            Self::dfs(&nums, 0, nums.len())
        }

        fn dfs(nums: &[i32], lo: usize, hi: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if lo >= hi {
                return None;
            }
            let mid = lo + (hi - lo) / 2;
            let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            root.borrow_mut().left = Self::dfs(nums, lo, mid);
            root.borrow_mut().right = Self::dfs(nums, mid + 1, hi);
            Some(root)
        }
    }
}
