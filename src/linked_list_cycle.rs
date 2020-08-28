/// leetcode没有提供给rust
pub mod solution_set {
    ///  ```java
    /// @Submission(date = "20200827", memory = 40.6, memoryBeatRate = 0, runtime = 5, runtimeBeatRate = 12, url = "https://leetcode-cn.com/submissions/detail/102136900/")
    /// public boolean hasCycle(ListNode head) {
    ///     Set<ListNode> nodes = new HashSet<>();
    ///     while (head != null) {
    ///         if (nodes.contains(head)) {
    ///             return true;
    ///         }
    ///         nodes.add(head);
    ///         head = head.next;
    ///     }
    ///     return false;
    /// }
    /// ```
    /// 
    /// 注意：ListNode没有重写equals, hashCode方法
    /// 
    /// 默认的Object#equals比较地址，#hashCode方法将返回唯一的code可以保证ListNode在set中唯一
    /// 
    /// jdk文档[hashCode](https://docs.oracle.com/en/java/javase/13/docs/api/java.base/java/lang/Object.html#hashCode())：`As far as is reasonably practical, the hashCode method defined by class Object returns distinct integers for distinct objects.`
    pub struct Solution;
}

pub mod solution_double_pointer {
    /// # 思路
    /// 
    /// 通过使用具有 不同速度 的快、慢两个指针遍历链表，空间复杂度可以被降低至 O(1)O(1)。慢指针每次移动一步，而快指针每次移动两步。
    ///
    /// 如果列表中不存在环，最终快指针将会最先到达尾部，此时我们可以返回 false。
    /// 
    /// 现在考虑一个环形链表，把慢指针和快指针想象成两个在环形赛道上跑步的运动员（分别称之为慢跑者与快跑者）。
    /// 而快跑者最终一定会追上慢跑者。这是为什么呢？考虑下面这种情况（记作情况 A）
    /// 
    /// - 假如快跑者只落后慢跑者一步，在下一次迭代中，它们就会分别跑了一步或两步并相遇。
    /// - 其他情况又会怎样呢？例如，我们没有考虑快跑者在慢跑者之后两步或三步的情况。但其实不难想到，因为在下一次或者下下次迭代后，又会变成上面提到的情况 A。
    /// 
    /// ```java
    /// @Submission(date = "20200827", memory = 40.2, memoryBeatRate = 22.33, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/102147200/")
    /// @Submission(date = "20200828", memory = 40, memoryBeatRate = 44.71, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/102425044/")
    /// // double pointer
    /// public boolean hasCycle(ListNode head) {
    ///     ListNode slow = head;
    ///     ListNode fast = head;
    ///     while (fast != null && fast.next != null) {
    ///         fast = fast.next.next;
    ///         slow = slow.next;
    ///         if (fast == slow) {
    ///             return true;
    ///         }
    ///     }
    ///     return false;
    /// }
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [环形链表](https://leetcode-cn.com/problems/linked-list-cycle/solution/huan-xing-lian-biao-by-leetcode/)
    /// - [O(1) Space Solution](https://leetcode.com/problems/linked-list-cycle/discuss/44489/O(1)-Space-Solution)
    /// 
    /// ### 复杂度
    /// 
    /// - 时间复杂度：O(n)
    /// - 空间复杂度：O(1)
    pub struct Solution;
}
