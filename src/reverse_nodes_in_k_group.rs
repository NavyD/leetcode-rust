pub mod solution_iterative {
    use crate::prelude::*;
    /// # 思路
    ///
    /// 链表分区为已翻转部分+待翻转部分+未翻转部分
    ///
    /// 翻转类似[reverse_linked_list](https://leetcode-cn.com/problems/reverse-linked-list/)，
    /// 通过pre节点找出第k个tail节点，切断反转并重建链表
    ///
    /// ```java
    /// // 移除
    ///             // 切断链表
    /// head = pre.next;
    /// ListNode next = tail.next;
    /// tail.next = null;
    /// ```
    ///
    /// 改进为不再先切断tail，而是用reverse head与nextHead关系会head.next=null切断，再用
    /// head.next = nextHead链接
    ///
    /// ![](https://pic.leetcode-cn.com/866b404c6b0b52fa02385e301ee907fc015742c3766c80c02e24ef3a8613e5ad-k%E4%B8%AA%E4%B8%80%E7%BB%84%E7%BF%BB%E8%BD%AC%E9%93%BE%E8%A1%A8.png)
    ///
    /// ```java
    /// // 改进为递归
    /// @Submission(date = "20200829", memory = 39.9, memoryBeatRate = 54, runtime = 1, runtimeBeatRate = 35, url = "https://leetcode-cn.com/submissions/detail/102745511/")
    /// @Submission(date = "20200830", memory = 40.1, memoryBeatRate = 17.08, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/103021922/")
    /// //iterative
    /// public ListNode reverseKGroup(ListNode head, int k) {
    ///     ListNode dummy = new ListNode(0);
    ///     dummy.next = head;
    ///     ListNode pre = dummy;
    ///     while (true) {
    ///         // find next head in k
    ///         head = pre.next;
    ///         ListNode nextHead = head;
    ///         for (int i = 0; i < k; i++) {
    ///             if (nextHead == null) {
    ///                 return dummy.next;
    ///             }
    ///             nextHead = nextHead.next;
    ///         }
    ///         // reverse
    ///         pre.next = reverse(head, nextHead);
    ///         // link reversed. now head is tail
    ///         head.next = nextHead;
    ///         pre = head;
    ///     }
    /// }
    ///
    /// private ListNode reverse(ListNode head, ListNode nextHead) {
    ///     ListNode prev = null;
    ///     while (head != nextHead) {
    ///         ListNode next = head.next;
    ///         head.next = prev;
    ///         prev = head;
    ///         head = next;
    ///     }
    ///     return prev;
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [图解k个一组翻转链表](https://leetcode-cn.com/problems/reverse-nodes-in-k-group/solution/tu-jie-kge-yi-zu-fan-zhuan-lian-biao-by-user7208t/)
    /// - [K 个一组翻转链表](https://leetcode-cn.com/problems/reverse-nodes-in-k-group/solution/k-ge-yi-zu-fan-zhuan-lian-biao-by-leetcode-solutio/)
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        #[allow(unused_variables)]
        pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            todo!()
        }
    }
}

pub mod solution_recursive {
    use crate::prelude::*;

    /// # 思路
    /// 
    /// rust所有权的方式不能在递归前反转链表，只能递归后返回反转的new_head作为next_head
    /// 与上个head反转链接 
    /// 
    /// `head -> ... -x-> next_head -> tail` 反转链接 `tail <- head <- ... <- next_head` 并返回next_head
    /// 
    /// ```rust,ignore
    /// Solution::reverse(head, next_head)
    /// // cannot move out of `next_head` 
    /// let next_head = Solution::reverse_k_group(next_head.take(), k);
    /// ```
    ///
    /// 先反转每段k个链表，递归回溯时连接相邻链表
    ///
    /// ![](https://pic.leetcode-cn.com/f63d5ca4d3f055ce8e4591c8bc51c288791f88da9ccec9617bc8bb51c26163a2.png)
    ///
    /// ```java
    /// @Submission(date = "20200829", memory = 40.2, memoryBeatRate = 8.88, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/102757084/")
    /// @Submission(date = "20200830", memory = 40.1, memoryBeatRate = 17.84, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/103019337/")
    /// public ListNode reverseKGroup(ListNode head, int k) {
    ///     if (head == null) {
    ///         return null;
    ///     }
    ///     ListNode nextHead = head;
    ///     for (int i = 0; i < k; i++) {
    ///         if (nextHead == null) {
    ///             return head;
    ///         }
    ///         nextHead = nextHead.next;
    ///     }
    ///     ListNode newHead = reverse(head, nextHead);
    ///     // head is tail
    ///     head.next = reverseKGroup(nextHead, k);
    ///     return newHead;
    /// }
    ///
    /// // 反转head->nextHead不包括nextHead
    /// private ListNode reverse(ListNode head, ListNode nextHead) {
    ///     ListNode prev = null;
    ///     while (head != nextHead) {
    ///         ListNode next = head.next;
    ///         head.next = prev;
    ///         prev = head;
    ///         head = next;
    ///     }
    ///     return prev;
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [递归思维：如何跳出细节？](https://leetcode-cn.com/problems/reverse-nodes-in-k-group/solution/di-gui-si-wei-ru-he-tiao-chu-xi-jie-by-labuladong/)
    /// - [递归java](https://leetcode-cn.com/problems/reverse-nodes-in-k-group/solution/di-gui-java-by-reedfan-2/)
    /// - [4ms](https://leetcode.com/submissions/api/detail/25/rust/4/)
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(log n)
    pub struct Solution;

    impl Solution {
        pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            let mut head = head;
            let mut next_head = &mut head;
            for _ in 0..k {
                if let Some(node) = next_head.as_mut() {
                    next_head = &mut node.next;
                } else {
                    return head;
                }
            }
            // 切断head -> ... -> tail -x-> next_head -> ...
            let next_head = Solution::reverse_k_group(next_head.take(), k);
            Solution::reverse(head, next_head)
        }
        
        /// head -> ... -x-> next_head -> tail 反转链接 tail <- head <- ... <- next_head 并返回new_head
        fn reverse(
            mut head: Option<Box<ListNode>>,
            mut next_head: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            while let Some(mut node) = head {
                head = node.next.take();
                node.next = next_head.take();
                next_head = Some(node);
            }
            next_head
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert_eq!(
                linkedlist![2, 1, 4, 3, 5],
                Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5], 2)
            );
        }
    }
}
