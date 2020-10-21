#![allow(dead_code)]

use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod solution_dfs {
    use super::*;
    const DELIMITER: &'static str = ",";
    const NONE: &'static str = "null";

    /// # 思路
    /// 
    /// DFS（递归）
    /// 
    /// 参考：
    /// 
    /// - [『手画图解』二叉树的序列化与反序列化 | 剖析两种解法](https://leetcode-cn.com/problems/serialize-and-deserialize-binary-tree/solution/shou-hui-tu-jie-gei-chu-dfshe-bfsliang-chong-jie-f/)
    /// 
    /// ### Submissions
    /// 
    /// date=20201021, mem=3.2, mem_beats=100, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/117405446/
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
            Self::de(&mut data.split(DELIMITER))
        }

        fn de<'a, T: Iterator<Item = &'a str>>(iter: &mut T) -> Option<Rc<RefCell<TreeNode>>> {
            iter.next().and_then(|s| s.parse::<i32>().ok()).map(|v| {
                let root = Rc::new(RefCell::new(TreeNode::new(v)));
                let temp = root.clone();
                let mut root_ref = temp.borrow_mut();
                root_ref.left = Self::de(iter);
                root_ref.right = Self::de(iter);
                root
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
