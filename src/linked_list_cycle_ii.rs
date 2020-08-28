/// 总结
/// 
/// 快慢双指针如何在第二次相遇时一定在起点，
/// 涉及到相关证明
/// 
/// 未提供rust
pub mod solution_set {
    /// # 思路
    /// 
    /// 用一个 Set 保存已经访问过的节点，遍历整个列表并返回第一个出现重复的节点。
    /// 
    /// ```java
    /// @Submission(date = "20200828", memory = 41.2, memoryBeatRate = 0, runtime = 5, runtimeBeatRate = 14, url = "https://leetcode-cn.com/submissions/detail/102397224/")
    /// public ListNode detectCycle(ListNode head) {
    ///     Set<ListNode> nodes = new HashSet<>();
    ///     while (head != null) {
    ///         if (nodes.contains(head)) {
    ///             return head;
    ///         }
    ///         nodes.add(head);
    ///         head = head.next;
    ///     }
    ///     return null;
    /// }
    /// ```
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;
}

pub mod solution_double_pointer {
    /// # 思路
    /// 
    /// 当链表中有环起点为start，fast,slow两指针在环中相遇在point，距离关系：`head->start == n*(point->point) + point->start`
    /// 
    /// ![](https://pic.leetcode-cn.com/2036dfe7e991f00dfb788a9b84a17bb6fac337e81c09bdf57e683d028a6952bc-%E6%9C%AA%E5%91%BD%E5%90%8D%E6%96%87%E4%BB%B6.png)
    /// 
    /// 相遇时：
    /// 
    /// slow走过的路程：x + y + n(y+z), n代表slow绕了n圈
    /// 
    /// fast走过的路程：x + y + m(y+z)，m代表fast饶了m圈
    /// 
    /// m > n
    /// 
    /// 因为fast速度是slow两倍：
    /// 
    /// 2(x + y + n(y + z)) = x + y + m(y + z)
    /// 
    /// x + y = (m - 2n)(y + z)
    /// 
    /// x = (m - 2n)(y + z) - y
    /// 
    /// y + z就是1圈，假设 o = m-2n，o是一个正整数，那么 x = o(y + z) -y
    /// 
    /// 如果o = 1，那么 x = z，和答主假设的情况一样
    /// 
    /// 如果o > 1，那么 x = (o-1)(y+z) + y + z - y, x = (o-1)(y+z) + z，即x的长度为o-1圈加上z
    /// 
    /// 所以，从第一阶段获得的相遇点，走过x距离机会到达环的起点。
    /// 
    /// ```java
    /// @Submission(date = "20200828", memory = 39.8, memoryBeatRate = 68.29, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/102414197/")
    /// public ListNode detectCycle(ListNode head) {
    ///     ListNode slow = head;
    ///     ListNode fast = head;
    ///     while (fast != null && fast.next != null) {
    ///         fast = fast.next.next;
    ///         slow = slow.next;
    ///         if (slow == fast) {
    ///             fast = head;
    ///             while (slow != fast) {
    ///                 slow = slow.next;
    ///                 fast = fast.next;
    ///             }
    ///             return fast;
    ///         }
    ///     }
    ///     return null;
    /// }
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [详细图解(肯定看的明白)](https://leetcode-cn.com/problems/linked-list-cycle-ii/solution/xiang-xi-tu-jie-ken-ding-kan-de-ming-bai-by-xixili/)
    /// - [详细图解(肯定看的明白) 证明](https://leetcode-cn.com/problems/linked-list-cycle-ii/solution/xiang-xi-tu-jie-ken-ding-kan-de-ming-bai-by-xixili/471381)
    /// - [环形链表 II（双指针法，清晰图解）](https://leetcode-cn.com/problems/linked-list-cycle-ii/solution/linked-list-cycle-ii-kuai-man-zhi-zhen-shuang-zhi-/)
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(1)
    pub struct Solution;
}