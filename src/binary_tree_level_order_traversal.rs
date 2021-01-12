use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_bfs {
    use super::*;
    /// # 思路
    ///
    /// ### Submissions
    ///
    /// date=20201220, mem=2.2, mem_beats=26, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132326653/
    ///
    /// date=20201221, mem=2.2, mem_beats=26, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132740574/
    /// 
    /// date=20210112, mem=2.1, mem_beats=91, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137813347/
    pub struct Solution;
    impl Solution {
        pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut res = vec![];
            if let Some(root) = root {
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(root);
                while !queue.is_empty() {
                    // access
                    let mut vals = vec![];
                    for _ in 0..queue.len() {
                        let node = queue.pop_front().unwrap();
                        let mut node = node.borrow_mut();
                        vals.push(node.val);
                        // next tier
                        if let Some(left) = node.left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = node.right.take() {
                            queue.push_back(right);
                        }
                    }
                    res.push(vals);
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_bfs::Solution::level_order);
    }

    fn test<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>>(func: F) {
        assert_eq!(func(None), vec![] as Vec<Vec<i32>>);
        assert_eq!(
            func(btree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
