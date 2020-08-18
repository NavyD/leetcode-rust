// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/// 总结
/// 
/// 写不出rust的递归链表方式
pub mod solution_points {
    use super::*;
    /// 
    /// 参考：[反转链表](https://leetcode-cn.com/problems/reverse-linked-list/solution/fan-zhuan-lian-biao-by-leetcode/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200818, mem=2.3, mem_beats=83.33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/99449141/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut pre_node = None;
            while let Some(mut node) = head {
                head = node.next;
                node.next = pre_node;
                pre_node = Some(node);
            }
            pre_node
        }

    }
}
