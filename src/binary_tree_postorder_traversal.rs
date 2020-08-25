
/// 总结
/// 
/// solution bfs和stack都不熟悉，5分钟没有想到，
/// 要注意与先序的left, right相反位置
/// 
/// 20200815
/// 
/// 第2次没什么问题
pub mod solution_recursive {
    use crate::helper::*;

    pub struct Solution;

    impl Solution {
        pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            Self::postorder(root, &mut vals);
            vals
        }

        fn postorder(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(root) = root {
                let root = root.borrow();
                Self::postorder(root.left.clone(), vals);
                Self::postorder(root.right.clone(), vals);
                vals.push(root.val);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(
                vec![3, 2, 1],
                Solution::postorder_traversal(convert_tree("[1,null,2,3]"))
            );
        }
    }
}

pub mod solution_bfs {
    use crate::helper::*;

    /// # 思路
    ///
    /// 在preorder的基础上对结果reverse()
    ///
    /// ### Submission
    ///
    /// date=20200814, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98194630/
    /// 
    /// date=20200815, mem=2.1, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98283919/
    /// 
    /// date=20200825, mem=2.1, mem_beats=14.29, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/101710776/
    ///
    /// ### 参考
    ///
    /// - [二叉树的后序遍历](https://leetcode-cn.com/problems/binary-tree-postorder-traversal/solution/er-cha-shu-de-hou-xu-bian-li-by-leetcode/)
    pub struct Solution;

    impl Solution {
        pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            if let Some(root) = root {
                let mut stack = std::collections::VecDeque::new();
                stack.push_back(root);
                while let Some(root) = stack.pop_back() {
                    let root = root.borrow();
                    vals.push(root.val);
                    if let Some(left) = root.left.clone() {
                        stack.push_back(left);
                    }
                    if let Some(right) = root.right.clone() {
                        stack.push_back(right);
                    }
                }
                vals.reverse();
            }
            vals
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(
                vec![3, 2, 1],
                Solution::postorder_traversal(convert_tree("[1,null,2,3]"))
            );
        }
    }
}

pub mod solution_stack {
    use crate::helper::*;

    /// # 思路
    /// 
    /// 递归方式stack，先root->right->left，再vals reverse得到后序.
    /// 
    /// 与先序的stack方式相反，后序要先出right将val值排到left前面，再到
    /// reverse时则left在前
    /// 
    /// ### Submission
    /// 
    /// date=20200814, mem=2.1, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98208131/
    /// 
    /// date=20200815, mem=1.9, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98284899/
    /// 
    /// date=20200825, mem=1.9, mem_beats=85.71, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/101709346/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            let mut stack = std::collections::VecDeque::new();
            let mut cur_node = root;
            while cur_node.is_some() || !stack.is_empty() {
                while let Some(node) = cur_node.clone() {
                    stack.push_back(node.clone());
                    let node = node.borrow();
                    vals.push(node.val);
                    cur_node = node.right.clone();
                }
                cur_node = stack.pop_back().unwrap().borrow().left.clone();
            }
            vals.reverse();
            vals
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(
                vec![3, 2, 1],
                Solution::postorder_traversal(convert_tree("[1,null,2,3]"))
            );

            assert_eq!(
                vec![1,2,3],
                Solution::postorder_traversal(convert_tree("[3,1,2]"))
            );
        }
    }
}
