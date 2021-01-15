use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_bfs {
    use super::*;
    /// # 思路
    /// 
    /// ### Submissions
    /// 
    /// date=20201225, mem=2.8, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133615166/
    /// 
    /// date=20201227, mem=2.9, mem_beats=60, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/134083520/
    /// 
    /// date=20210115, mem=2.8, mem_beats=88, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138593407/
    pub struct Solution;

    impl Solution {
        pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut res = vec![];
            if let Some(root) = root {
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(root);
                while !queue.is_empty() {
                    let mut max_val = std::i32::MIN;
                    for _ in 0..queue.len() {
                        let node = queue.pop_front().unwrap();
                        let mut node = node.borrow_mut();
                        if node.val > max_val {
                            max_val = node.val;
                        }
                        if let Some(left) = node.left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = node.right.take() {
                            queue.push_back(right);
                        }
                    }
                    res.push(max_val);
                }
            }
            res
        }
    }
}

pub mod solution_dfs {
    use super::*;

    /// # 思路
    /// 
    /// dfs利用每层的val并在后面的同层dfs比较大小替换。level从0开始，
    /// 如果res.len()==level表示第level层首次添加val，否则比较大小替换
    /// 
    /// ### Submissions
    /// 
    /// date=20201225, mem=3.1, mem_beats=14, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133620233/
    /// 
    /// date=20201227, mem=2.9, mem_beats=70, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/134082381/
    /// 
    /// date=20210115, mem=2.8, mem_beats=77, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138594427/
    pub struct Solution;

    impl Solution {
        pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            fn _dfs(root: Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<i32>) {
                if let Some(root) = root {
                    let mut root = root.borrow_mut();
                    if res.len() <= level {
                        res.push(root.val);
                    } else {
                        res[level] = res[level].max(root.val);
                    }
                    _dfs(root.left.take(), level + 1, res);
                    _dfs(root.right.take(), level + 1, res);
                }
            }
            let mut res = vec![];
            _dfs(root, 0, &mut res);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_bfs::Solution::largest_values);
        test(solution_dfs::Solution::largest_values);
    }
    
    fn test<F: Fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>>(f: F) {
        assert_eq!(f(btree![1, 3, 2, 5, 3, null, 9]), vec![1, 3, 9]);
        assert_eq!(f(btree![1, 2, 3]), vec![1, 3]);
        assert_eq!(f(btree![1,null,2]), vec![1, 2]);
        assert_eq!(f(btree![]), vec![]);
    }
    
}
