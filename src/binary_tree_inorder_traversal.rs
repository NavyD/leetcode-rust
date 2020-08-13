use crate::helper::*;

pub mod solution_dfs {
    use super::*;

    /// ### Submissions
    /// 
    /// date=20200812, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/97279521/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(log N)。不考虑结果vals
    pub struct Solution;

    impl Solution {
        pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            Self::inorder(root, &mut vals);
            vals
        }

        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(root) = root {
                let root = root.borrow();
                Self::inorder(root.left.clone(), vals);
                vals.push(root.val);
                Self::inorder(root.right.clone(), vals);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert_eq!(vec![1,3,2], Solution::inorder_traversal(convert_tree("[1,null,2,3]")));
        }
    }
}

/// 第1次
/// 
/// 只记得通用递归，stack忘记了
pub mod solution_stack {
    use super::*;

    /// ### Submissions
    /// 
    /// date=20200812, mem=2, mem_beats=100, runtime=0, runtime_beats=2, url=https://leetcode-cn.com/submissions/detail/97285664/
    /// 
    /// date=20200813, mem=2, mem_beats=100, runtime=0, runtime_beats=2, url=https://leetcode-cn.com/submissions/detail/97601369/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            let mut stack = std::collections::VecDeque::new();
            let mut cur_node = root;
            while cur_node.is_some() || !stack.is_empty() {
                // push left
                while let Some(root) = cur_node.clone() {
                    cur_node = root.borrow().left.clone();
                    stack.push_back(root);
                }
                // pop left
                let root = stack.pop_back().unwrap();
                vals.push(root.borrow().val);
                // right as new root
                cur_node = root.borrow().right.clone();
            }
            vals
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert_eq!(vec![1,3,2], Solution::inorder_traversal(convert_tree("[1,null,2,3]")));
        }
    }
}
