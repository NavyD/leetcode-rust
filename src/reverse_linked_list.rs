/// 总结
///
/// 写不出rust的递归链表方式，现在是真不清楚
/// 
/// java可以写出递归方式
pub mod solution_points {
    use crate::helper::*;
    ///
    /// 参考：[反转链表](https://leetcode-cn.com/problems/reverse-linked-list/solution/fan-zhuan-lian-biao-by-leetcode/)
    ///
    /// ### Submissions
    ///
    /// date=20200818, mem=2.3, mem_beats=83.33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/99449141/
    /// 
    /// date=20200819, mem=2.4, mem_beats=66.67, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/99630592/
    /// 
    /// date=20200825, mem=2.5, mem_beats=5.56, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/101696406/
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

pub mod solution_recursive {
    use crate::helper::*;

    /// # 思路
    /// 
    /// 递归。在递归回溯时做到`cur.next = pre; pre.next = null`
    /// 
    /// ```java
    /// public ListNode reverseList(ListNode head) {
    ///     if (head == null || head.next == null) {
    ///         return head;
    ///     }
    ///     ListNode cur = reverseList(head.next);
    ///     // next = pre
    ///     head.next.next = head;
    ///     head.next = null;
    ///     return cur;
    /// }
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [动画演示+多种解法 206. 反转链表](https://leetcode-cn.com/problems/reverse-linked-list/solution/dong-hua-yan-shi-206-fan-zhuan-lian-biao-by-user74/)
    /// - [rust递归求解](https://leetcode-cn.com/problems/reverse-linked-list/solution/di-gui-qiu-jie-by-iceleo/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200819, mem=2.4, mem_beats=66.67, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/99628172/
    pub struct Solution;

    impl Solution {
        pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            Self::rev(head, None)
        }

        fn rev(head: Option<Box<ListNode>>, curr: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut head) = head {
                let next = std::mem::replace(&mut head.next, curr);
                Self::rev(next, Some(head))
            } else {
                curr
            }
        }
    }
}
