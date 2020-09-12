/// 总结
///
/// 写不出rust的递归链表方式，现在是真不清楚
/// 
/// java可以写出递归方式
/// 
/// 20200827
/// 
/// rust只能用重建链表方式实现
pub mod solution_points {
    use crate::prelude::*;
    /// # 思路
    /// 
    /// 用pre_node新建链表
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
    /// date=20200903, mem=2.4, mem_beats=33.33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104343441/
    /// 
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut node) = head {
                head = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
            prev
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(Solution::reverse_list(linkedlist![1, 2, 3, 4, 5]), linkedlist![5, 4, 3, 2, 1]);
        }
    }
}

pub mod solution_recursive {
    use crate::prelude::*;

    /// # 思路
    /// 
    /// 递归
    /// 
    /// rust用prev移动重建新链表，与迭代方式一样。不能与java版的一样，next不在回溯后
    /// 存在
    /// 
    /// java是在原链表tail时移动指针next->head回溯，只要移动指针
    /// reverseList在回溯时一直都返回的是head->tail的tail没变过，原tail是新head
    /// 
    /// ```java
    /// @Submission(date = "20200825", memory = 40, memoryBeatRate = 9.5, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/101700482/")
    /// @Submission(date = "20200827", memory = 40.1, memoryBeatRate = 5.19, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/102105920/")
    /// @Submission(date = "20200903", memory = 40, memoryBeatRate = 9.52, runtime = 0, runtimeBeatRate = 100, url = "https://leetcode-cn.com/submissions/detail/104356834/")
    /// public ListNode reverseList(ListNode head) {
    ///     if (head == null || head.next == null) {
    ///         return head;
    ///     }
    ///     ListNode tail = reverseList(head.next);
    ///     // next = pre
    ///     head.next.next = head;
    ///     head.next = null;
    ///     return tail;
    /// }
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [动画演示+多种解法 206. 反转链表](https://leetcode-cn.com/problems/reverse-linked-list/solution/dong-hua-yan-shi-206-fan-zhuan-lian-biao-by-user74/)
    /// - [rust递归求解](https://leetcode-cn.com/problems/reverse-linked-list/solution/di-gui-qiu-jie-by-iceleo/)
    /// - [Rust recursive](https://leetcode.com/problems/reverse-linked-list/discuss/518917/Rust-recursive)
    /// 
    /// ### Submissions
    /// 
    /// date=20200819, mem=2.4, mem_beats=66.67, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/99628172/
    /// 
    /// date=20200827, mem=2.6, mem_beats=9.52, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/102108428/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(log N)
    pub struct Solution;

    impl Solution {
        pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            Self::reverse(head, None)
        }

        fn reverse(head: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut head) = head {
                let next = head.next.take();
                head.next = prev;
                Self::reverse(next, Some(head))
            } else {
                prev
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(Solution::reverse_list(linkedlist![1, 2, 3, 4, 5]), linkedlist![5, 4, 3, 2, 1]);
        }
    }
}
