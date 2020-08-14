/// 总结
/// 
/// stack与用bfs的方式相比，该方式更符合递归的思路，
/// 参考inorder的stack思路。
/// 
/// bfs通常是用访问当前层所有结点，可这里用到了stack递归，
/// 不好理解，但是这里bfs的代码更简洁。
/// 
/// ## 20200814
/// 
/// 对solution_stack方式不清楚
/// 
/// - stack push root与 vals push root.val先后顺序不清
/// - root.right作为root访问，做出`stack.push_back(stack.pop_back().unwrap().borrow().right.clone())`
/// 
/// 对solution_bfs要注意先push_back(right)，再push left，下次先访问pop_back是left
/// 否则是right了
pub mod solution_dfs {
    use crate::helper::*;

    /// ### Submission
    ///
    /// date=20200813, mem=2.1, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/97572489/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            Self::preorder(root, &mut vals);
            vals
        }

        pub fn preorder(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(root) = root {
                let root = root.borrow();
                vals.push(root.val);
                Self::preorder(root.left.clone(), vals);
                Self::preorder(root.right.clone(), vals);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(
                vec![1, 2, 3],
                Solution::preorder_traversal(convert_tree("[1,null,2,3]"))
            );
        }
    }
}

pub mod solution_stack {
    use crate::helper::*;

    /// # 思路
    /// 
    /// 模拟栈的工作方式，先入left -> 访问root -> 入right
    /// 
    /// ### Submission
    ///
    /// date=20200813, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/97579046/
    ///
    /// date=20200814, mem=2.1, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98199464/
    /// 
    /// ### 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            let mut stack = std::collections::VecDeque::new();
            let mut cur_node = root;
            while cur_node.is_some() || !stack.is_empty() {
                while let Some(root) = cur_node.clone() {
                    stack.push_back(root.clone());
                    let root = root.borrow();
                    vals.push(root.val);
                    cur_node = root.left.clone();
                }
                cur_node = stack.pop_back().unwrap().borrow().right.clone();
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
                vec![1, 2, 3],
                Solution::preorder_traversal(convert_tree("[1,null,2,3]"))
            );
        }
    }
}

pub mod solution_bfs {
    use crate::helper::*;

    /// # 思路
    /// 
    /// 以bfs的方式入root结点，访问pop root，再入root.right -> root.left
    /// 
    /// ### 参考
    /// 
    /// [二叉树的前序遍历](https://leetcode-cn.com/problems/binary-tree-preorder-traversal/solution/er-cha-shu-de-qian-xu-bian-li-by-leetcode/)
    /// 
    /// ### Submission
    ///
    /// date=20200813, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/97585874/
    /// 
    /// date=20200814, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98202468/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut vals = vec![];
            if let Some(root) = root {
                let mut stack = std::collections::VecDeque::new();
                stack.push_back(root.clone());
                while let Some(root) = stack.pop_back() {
                    let root = root.borrow();
                    vals.push(root.val);
                    if let Some(right) = root.right.clone() {
                        stack.push_back(right);
                    }
                    if let Some(left) = root.left.clone() {
                        stack.push_back(left);
                    }
                }
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
                vec![1, 2, 3],
                Solution::preorder_traversal(convert_tree("[1,null,2,3]"))
            );
        }
    }
}
