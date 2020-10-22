use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_dfs {
    use super::*;

    pub struct Solution;

    impl Solution {
        // error
        pub fn lowest_common_ancestor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if root.as_ref().map_or(true, |root| {
                let root_val = root.borrow().val;
                p.as_ref().map_or(false, |p| p.borrow().val == root_val)
                    || q.as_ref().map_or(false, |q| q.borrow().val == root_val)
            }) {
                return root;
            }
            let left = Self::lowest_common_ancestor(
                root.clone().and_then(|root| root.borrow().left.clone()),
                p.clone(),
                q.clone(),
            );
            let right = Self::lowest_common_ancestor(
                root.clone().and_then(|root| root.borrow().right.clone()),
                p,
                q,
            );
            if left.is_none() && right.is_none() {
                None
            } else if left.is_none() {
                right
            } else if right.is_none() {
                left
            } else {
                root
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::lowest_common_ancestor);
    }

    fn test<F>(func: F)
    where
        F: Fn(
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>>,
    {
        assert_eq!(
            func(
                btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
                btree![5],
                btree![1]
            )
            .unwrap()
            .borrow()
            .val,
            3
        );

        assert_eq!(
            func(
                btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
                btree![5],
                btree![4]
            )
            .unwrap()
            .borrow()
            .val,
            btree![3].unwrap().borrow().val
        );
    }
}
