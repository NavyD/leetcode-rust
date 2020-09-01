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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

pub mod solution_dfs {
    use super::*;
    /// # 思路
    /// 
    /// 递归计算出深度
    /// 
    /// ## Submissions
    /// 
    /// date=20200713, mem=2.7, mem_beats=80.95, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/365857015/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(log N)
    /// 
    pub struct Solution;
    
    impl Solution {
        pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let None = root {
                return 0;
            } else {
                let root = root.to_owned().unwrap();
                let left_count = Self::max_depth(root.borrow().left.clone()) + 1;
                let right_count = Self::max_depth(root.borrow().right.clone()) + 1;
                return left_count.max(right_count)
            }
        }
    }
}

pub mod solution_bfs {
    use super::*;
    /// # 思路
    /// 
    /// 一层一层向下计算
    /// 
    /// 如何遍历平级结点？
    /// 
    /// 注意在平级结点中存在后面子结点push_back，不能pop出子结点作为平级结点用，
    /// 应该用size个数限制
    /// 
    /// ## Submissions
    /// 
    /// date=20200713, mem=2.5， mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/365882616/
    /// 
    /// author=makuiyu, references=https://leetcode.com/problems/maximum-depth-of-binary-tree/discuss/34207/My-code-of-C%2B%2B-Depth-first-search-and-Breadth-first-search
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;
    
    impl Solution {
        pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back(root);
            let mut count = 0;
            while let Some(Some(_)) = queue.front() {
                count += 1;
                for _ in 0..queue.len() {
                    // pop_front保证不将push_back的子结点作为平级结点计算
                    if let Some(Some(node)) = queue.pop_front() {
                        let node = node.borrow();
                        if node.left.is_some() {
                            queue.push_back(node.left.to_owned());
                        }
                        if node.right.is_some() {
                            queue.push_back(node.right.to_owned());
                        }
                    }
                }
            }
            count
        }
    }
}
