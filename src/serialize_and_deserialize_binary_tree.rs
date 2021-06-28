#![allow(dead_code)]

use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

const DELIMITER: &'static str = ",";
const NONE: &'static str = "null";

pub mod solution_dfs {
    use super::*;

    /// # 思路
    ///
    /// 使用前序 DFS（递归）
    ///
    /// 选择前序遍历是因为 根|左|右 的打印顺序，在反序列化时更容易定位出根节点的值。
    ///
    /// 注意：DELIMITER
    ///
    /// 参考：
    ///
    /// - [『手画图解』二叉树的序列化与反序列化 | 剖析两种解法](https://leetcode-cn.com/problems/serialize-and-deserialize-binary-tree/solution/shou-hui-tu-jie-gei-chu-dfshe-bfsliang-chong-jie-f/)
    ///
    /// ### Submissions
    ///
    /// date=20201021, mem=3.2, mem_beats=100, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/117405446/
    ///
    /// date=20201022, mem=3.1, mem_beats=100, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/117716368/
    ///
    /// date=20201031, mem=3, mem_beats=66.67, runtime=12, runtime_beats=33.33, url=https://leetcode-cn.com/submissions/detail/119963318/
    struct Codec {}

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl Codec {
        fn new() -> Self {
            Self {}
        }

        fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
            root.map_or(NONE.to_string() + DELIMITER, |root| {
                let root = root.borrow();
                root.val.to_string()
                    + DELIMITER
                    + &self.serialize(root.left.clone())
                    + &self.serialize(root.right.clone())
            })
        }

        fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
            // data.remove(data.len() - DELIMITER.len());
            self.de(&mut data.split(DELIMITER))
        }

        fn de<'a>(
            &self,
            iter: &mut impl Iterator<Item = &'a str>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            iter.next().and_then(|v| v.parse().ok()).map(|v| {
                let mut root = TreeNode::new(v);
                root.left = self.de(iter);
                root.right = self.de(iter);
                Rc::new(RefCell::new(root))
            })
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let data = btree![1, 2, 3, null, null, 4, 5];
            let codec = solution_dfs::Codec::new();
            assert_eq!(codec.deserialize(codec.serialize(data.clone())), data);
        }
    }
}

mod solution_bfs {
    use super::*;

    /// # 思路
    ///
    /// 层序序列化
    ///
    /// 注意不需要在一般的层序遍历中`for _ in `，只要用`while let Some(root) = queue.pop_back()`就可以连续
    /// 找到所有节点
    ///
    /// 在反序列化时queue要保存d所有节点，包括None，所以用`queue.push_back(root: Option<Rc<RefCell<TreeNode>>>)`
    ///
    /// 参考：
    ///
    /// - [『手画图解』二叉树的序列化与反序列化 | 剖析两种解法](https://leetcode-cn.com/problems/serialize-and-deserialize-binary-tree/solution/shou-hui-tu-jie-gei-chu-dfshe-bfsliang-chong-jie-f/)
    ///
    /// ### Submissions
    ///
    /// date=20201022, mem=3, mem_beats=100, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/117749759/
    struct Codec {}

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl Codec {
        fn new() -> Self {
            Self {}
        }

        fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            let mut data = String::new();
            while let Some(root) = queue.pop_front() {
                if let Some(root) = root {
                    let root = root.borrow();
                    queue.push_back(root.left.clone());
                    queue.push_back(root.right.clone());
                    data += &(root.val.to_string() + DELIMITER);
                } else {
                    data = data + NONE + DELIMITER;
                }
            }
            data
        }

        fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
            let mut iter = data.split(DELIMITER);
            let root = iter
                .next()
                .and_then(|s| s.parse::<i32>().ok())
                .map(|v| Rc::new(RefCell::new(TreeNode::new(v))));
            if let Some(root) = root.clone() {
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(root);
                while let Some(root) = queue.pop_front() {
                    root.borrow_mut().left =
                        iter.next().and_then(|s| s.parse::<i32>().ok()).map(|v| {
                            let left = Rc::new(RefCell::new(TreeNode::new(v)));
                            queue.push_back(left.clone());
                            left
                        });
                    root.borrow_mut().right =
                        iter.next().and_then(|s| s.parse::<i32>().ok()).map(|v| {
                            let right = Rc::new(RefCell::new(TreeNode::new(v)));
                            queue.push_back(right.clone());
                            right
                        });
                }
            }
            root
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let data = btree![1, 2, 3, null, null, 4, 5];
            let codec = Codec::new();
            assert_eq!(codec.deserialize(codec.serialize(data.clone())), data);
        }
    }
}
