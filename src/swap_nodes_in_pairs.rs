/// 总结
/// 
/// rust在链表的操作上有许多不同，所有权、借用限制在
/// java方式上使用
pub mod solution_iterative {
    use crate::helper::*;
    
    /// # 思路
    /// 
    /// 由于 Rust 的所有权和借用系统，相比按照 java 思路的*修改节点上指针的指向*，使用*将原链表上的节点移动过来重新构造新的链表*思路更加合适。
    /// 
    /// ```java
    /// // iterative
    /// @Submission(date = "20200826", memory = 37.3, memoryBeatRate = 67.97, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/101838737/")
    /// public ListNode swapPairs(ListNode head) {
    ///     ListNode dummy = new ListNode(0);
    ///     dummy.next = head;
    ///     ListNode cur = dummy;
    ///     while (head != null && head.next != null) {
    ///         ListNode first = head;
    ///         ListNode second = head.next;
    ///         // swap
    ///         cur.next = second;
    ///         first.next = second.next;
    ///         second.next = first;
    ///         
    ///         cur = first;
    ///         head = first.next;
    ///     }
    ///     return dummy.next;
    /// }
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [循环解法系列：简单高效，含图解](https://leetcode-cn.com/problems/swap-nodes-in-pairs/solution/bi-jiao-zhi-jie-gao-xiao-de-zuo-fa-han-tu-jie-by-w/)
    /// - [两两交换链表中的节点](https://leetcode-cn.com/problems/swap-nodes-in-pairs/solution/liang-liang-jiao-huan-lian-biao-zhong-de-jie-di-19/)
    /// - [rust「非递归法」移动原链表节点到新链表上返回](https://leetcode-cn.com/problems/swap-nodes-in-pairs/solution/fei-di-gui-fa-yi-dong-yuan-lian-biao-jie-dian-dao-/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200826, mem=2, mem_beats=66.67, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/101843120/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut head = head;
            let mut dummy = ListNode::new(0);
            let mut tail = &mut dummy;
            while let Some(mut first) = head {
                // first.next = none
                head = first.next.take();
                if let Some(mut second) = head {
                    // head = second.next; second.next = none;
                    head = second.next.take();
                    // link new node
                    second.next = Some(first);
                    tail.next = Some(second);
                    // move tail 2 step
                    tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
                } else {
                    tail.next = Some(first);
                    // move tail 1 step
                    tail = tail.next.as_mut().unwrap();
                }
            }
            dummy.next
        }
    }
}

pub mod solution_recursive {
    use crate::helper::*;

    /// # 思路
    /// 
    /// 更新每两个点的链表形态完成整个链表的调整
    /// 
    /// - 调用单元：设需要交换的两个点为 head 和 next，head 连接后面交换完成的子链表，next 连接 head，完成交换
    /// - 终止条件：head 为空指针或者 next 为空指针，也就是当前无节点或者只有一个节点，无法进行交换
    /// - 返回值：交换完成的子链表
    /// 
    /// [画解](swap_nodes_in_pairs.drawio.png)
    /// 
    /// 
    /// ```java
    /// @Submission(date = "20200826", memory = 37.4, memoryBeatRate = 51.44, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/101834628/")
    /// public ListNode swapPairs(ListNode head) {
    ///     if (head == null || head.next == null)
    ///         return head;
    ///     ListNode next = head.next;
    ///     ListNode subHead = swapPairs(next.next);
    ///     // swap
    ///     head.next = subHead;
    ///     next.next = head;
    ///     return next;
    /// }
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [画解算法：24. 两两交换链表中的节点](https://leetcode-cn.com/problems/swap-nodes-in-pairs/solution/hua-jie-suan-fa-24-liang-liang-jiao-huan-lian-biao/)
    /// - [Rust 0ms 100%](https://leetcode.com/problems/swap-nodes-in-pairs/discuss/233501/Rust-0ms-100)
    /// - [Rust recursion solution, clean code](https://leetcode.com/problems/swap-nodes-in-pairs/discuss/540192/Rust-recursion-solution-clean-code)
    /// 
    /// ### Submissions
    /// 
    /// date=20200826, mem=2.1, mem_beats=33.33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/101849080/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut head) = head {
                if let Some(mut next) = head.next {
                    let sub_node = Self::swap_pairs(next.next);
                    head.next = sub_node;
                    next.next = Some(head);
                    Some(next)
                } else {
                    Some(head)
                }
            } else {
                None
            }
            // head moved
            // return head;
        }

        /// java风格
        /// 
        /// date=20200826, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/101850473/
        pub fn swap_pairs_java(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() || head.as_ref().unwrap().next.is_none() {
                return head;
            }
            let mut head = head.unwrap();
            let mut next = head.next.unwrap();
            let sub_node = Self::swap_pairs_java(next.next);
            head.next = sub_node;
            next.next = Some(head);
            Some(next)
        }
    }
}