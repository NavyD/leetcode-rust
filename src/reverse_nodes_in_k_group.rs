pub mod solution_iterative {
    use crate::helper::*;
    /// # 思路
    /// 
    /// 链表分区为已翻转部分+待翻转部分+未翻转部分
    /// 
    /// 翻转类似[reverse_linked_list](https://leetcode-cn.com/problems/reverse-linked-list/)，
    /// 通过pre节点找出第k个tail节点，切断反转并重建链表
    /// 
    /// ![](https://pic.leetcode-cn.com/866b404c6b0b52fa02385e301ee907fc015742c3766c80c02e24ef3a8613e5ad-k%E4%B8%AA%E4%B8%80%E7%BB%84%E7%BF%BB%E8%BD%AC%E9%93%BE%E8%A1%A8.png)
    /// 
    /// ```java
    /// // 还有改进的地步，像递归一样
    /// @Submission(date = "20200829", memory = 39.9, memoryBeatRate = 54, runtime = 1, runtimeBeatRate = 35, url = "https://leetcode-cn.com/submissions/detail/102745511/")
    /// public ListNode reverseKGroup(ListNode head, int k) {
    ///     ListNode dummy = new ListNode(0);
    ///     dummy.next = head;
    ///     ListNode pre = dummy;
    ///     while (head != null) {
    ///         ListNode tail = pre;
    ///         // 找到第k个节点tail
    ///         for (int i = 0; i < k; i++) {
    ///             if ((tail = tail.next) == null) {
    ///                 return dummy.next;
    ///             }
    ///         }
    ///         // 切断链表
    ///         head = pre.next;
    ///         ListNode next = tail.next;
    ///         tail.next = null;
    ///         // head is tail
    ///         pre.next = reverseList(head);
    ///         // 重建 
    ///         head.next = next;
    ///         pre = head;
    ///     }
    ///     return dummy.next;
    /// }
    ///
    /// private ListNode reverseList(ListNode head) {
    ///     ListNode prev = null;
    ///     while (head != null) {
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

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

pub mod solution_recursive {
    use crate::helper::*;

    /// # 思路
    /// 
    /// 先反转每段k个链表，递归回溯时连接相邻链表
    /// 
    /// ![](https://pic.leetcode-cn.com/f63d5ca4d3f055ce8e4591c8bc51c288791f88da9ccec9617bc8bb51c26163a2.png)
    /// 
    /// ```java
    /// @Submission(date = "20200829", memory = 40.2, memoryBeatRate = 8.88, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/102757084/")
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
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n)
    /// - 空间：O(log n)
    pub struct Solution;

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}